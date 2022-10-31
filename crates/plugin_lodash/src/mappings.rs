use std::{
  fs,
  path::{Path, PathBuf},
};

use nodejs_resolver::{ResolveResult, Resolver};
use shared::{
  ahash::AHashMap,
  anyhow,
  serde::{Deserialize, Serialize},
  swc_core::common::sync::Lazy,
};

use crate::error::{ResolveError, ResolveErrorKind};

pub type ModuleId = String; // lodash, lodash-es, lodash-compat
pub type Mappings = AHashMap<ModuleId, ModuleMap>; // lodash -> [...], lodash-es -> [...]

#[derive(Debug)]
pub struct Package {
  /// When find module, try this first
  /// some pkg has main: ./lib/main.js,
  /// also has module: ./es/main.js
  /// when we find path using hashmap, we iterator hashmap, the order is
  /// not exact, we may get ./es/main.js, and we may get ./lib/main.js also
  ///
  /// However the main field should matters more due to babel-plugin-lodash
  pub main_path: String,
  pub id: String,
  pub base: String,
  pub pkg_path: PathBuf,
}

impl Package {
  pub fn new(pwd: &Path, source: &str, base: &str) -> anyhow::Result<Self> {
    let main_path = resolve(source, pwd)?;
    let pkg_root =
      get_pkg_root(main_path).unwrap_or_else(|| panic!("Cannot find package root for {}", source));

    let pkg_json = get_pkg_json(&pkg_root)?;

    let mut main_path = pkg_json.main.unwrap_or_else(|| String::from("index.js"));
    if main_path.starts_with("./") {
      main_path = main_path.strip_prefix("./").unwrap().into();
    };
    let mut main_path = PathBuf::from(&main_path);
    main_path.pop();

    Ok(Self {
      main_path: main_path.to_string_lossy().to_string(),
      id: source.into(),
      base: base.into(),
      pkg_path: pkg_root,
    })
  }

  pub fn find_module<'a>(&self, mappings: &'a Mappings, name: &str) -> Option<&'a PathBuf> {
    // For ```import { map } from 'lodash'```
    // We find through iterating `lodash/bar/map.js`, `lodash/foo/map.js`, `lodash/map.js`, ...

    let module_map = mappings.get(&self.id)?;

    if !self.base.is_empty() {
      module_map
        .get(&self.base)
        .and_then(|pairs| pairs.get(&name.to_lowercase()))
    } else {
      // Firstly we should look for main_path.
      for (_, pairs) in module_map
        .iter()
        .filter(|(dir, _)| dir.as_str() == self.main_path)
      {
        if let Some(path) = pairs.get(&name.to_lowercase()) {
          return Some(path);
        };
      }

      // Next we look for other path
      for (_, pairs) in module_map
        .iter()
        .filter(|(dir, _)| dir.as_str() != self.main_path)
      {
        if let Some(path) = pairs.get(&name.to_lowercase()) {
          return Some(path);
        }
      }
      None
    }
  }
}

pub type ModuleMap = AHashMap<String, Pairs>; // lib -> [...], es -> [...], dist -> [...]
pub type Pairs = AHashMap<String, PathBuf>; // camelcase -> camelCase, kebabcase -> kebabCase

static RESOLVER: Lazy<Resolver> = Lazy::new(|| Resolver::new(Default::default()));

pub fn build_mappings<'a>(
  ids: impl Iterator<Item = &'a str>,
  root: &Path,
) -> anyhow::Result<Mappings> {
  let mut mappings = Mappings::default();

  for id in ids {
    let module_root = resolve(id, root);
    if module_root.is_err() {
      continue;
    }

    let pkg_root = get_pkg_root(module_root.unwrap());
    if matches!(pkg_root, None) {
      println!("Module {} not found. Skipped", id);
      continue;
    }

    mappings.insert(id.into(), init_module_map(pkg_root.unwrap())?);
  }

  Ok(mappings)
}

fn resolve(id: &str, pwd: &Path) -> Result<PathBuf, ResolveError> {
  match RESOLVER.resolve(pwd, id) {
    Ok(info) => match info {
      ResolveResult::Info(info) => Ok(info.path),
      ResolveResult::Ignored => Err(ResolveError::new(id.into(), ResolveErrorKind::ShouldIgnore)),
    },
    Err(_) => Err(ResolveError::new(
      id.into(),
      ResolveErrorKind::ModuleNotFound,
    )),
  }
}

fn get_pkg_root(mut module_root: PathBuf) -> Option<PathBuf> {
  while fs::read(module_root.as_path().join("package.json")).is_err() {
    if !module_root.pop() {
      return None;
    };
  }

  Some(module_root)
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "shared::serde")]
struct PkgJson {
  name: String,
  main: Option<String>,
}

fn init_module_map(pkg_root: PathBuf) -> anyhow::Result<ModuleMap> {
  let mut module_dir_map = ModuleMap::default();

  let mut dir_paths: Vec<_> = glob::glob(&format!("{}/**", pkg_root.display()))?.collect();
  dir_paths.push(Ok(pkg_root.clone()));

  for dir in dir_paths {
    let dir = dir?;

    // make it relative
    let base = dir
      .as_path()
      .strip_prefix(&pkg_root)?
      .to_string_lossy()
      .replace(r"\\", "/")
      .to_string();

    module_dir_map.insert(base, build_pairs(&pkg_root, &dir)?);
  }

  Ok(module_dir_map)
}

fn get_pkg_json(module_root: &Path) -> anyhow::Result<PkgJson> {
  let pkg_json_path = Path::new(&module_root).join("package.json");

  Ok(shared::serde_json::from_slice::<PkgJson>(
    fs::read(&pkg_json_path)?.as_slice(),
  )?)
}

fn build_pairs(pkg_root: &Path, dir_path: &Path) -> anyhow::Result<Pairs> {
  let mut pairs = Pairs::default();

  let files = glob::glob(&format!("{}/*.js", dir_path.display()))?;

  for file_path in files {
    let file_path = file_path?;
    let file_name = file_path.strip_prefix(dir_path).unwrap();
    let name = file_name.to_string_lossy().to_string().replace(".js", "");
    let file_relative_path = file_path
      .strip_prefix(pkg_root)
      .unwrap()
      .to_str()
      .unwrap()
      .strip_suffix(".js")
      .unwrap();

    pairs.insert(name.to_lowercase(), PathBuf::from(file_relative_path));
  }

  Ok(pairs)
}
