use std::{
  fs,
  path::{Path, PathBuf, MAIN_SEPARATOR},
};

use nodejs_resolver::{ResolveResult, Resolver};
use shared::{
  ahash::AHashMap,
  anyhow,
  serde::{Deserialize, Serialize},
  swc_core::{common::sync::Lazy, ecma::atoms::JsWord},
};

use crate::error::{ResolveError, ResolveErrorKind};

pub type ModuleId = String; // lodash, lodash-es
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

  pub fn find_module<'a>(&self, mappings: &'a Mappings, name: &str) -> Option<&'a String> {
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
pub type Pairs = AHashMap<String, String>; // camelcase -> camelCase, kebabcase -> kebabCase

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

  let mut dir_paths: Vec<_> = fs::read_dir(&pkg_root)?
    .map(|item| item.unwrap().path())
    .filter(|p| p.is_dir())
    .collect();
  dir_paths.push(pkg_root.clone());

  for dir in dir_paths {
    // make it relative
    let base = dir
      .as_path()
      .strip_prefix(&pkg_root)?
      .to_string_lossy()
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

  let files = fs::read_dir(&dir_path)?
    .map(|it| it.unwrap().path())
    .filter(|it| it.extension().map(|ext| ext == "js").unwrap_or(false));

  for file_path in files {
    // in windows, file_name: a\\b\\c.js
    let file_name = file_path.strip_prefix(dir_path).unwrap();
    let name = file_name.to_string_lossy().to_string().replace(".js", "");
    let file_relative_path = file_path
      .strip_prefix(pkg_root)
      .unwrap()
      .to_str()
      .unwrap()
      .strip_suffix(".js")
      .unwrap()
      .replace(MAIN_SEPARATOR, "/");

    pairs.insert(name.to_lowercase(), file_relative_path);
  }

  Ok(pairs)
}

pub fn build_pkg_map(cwd: &Path, mappings: &Mappings) -> AHashMap<JsWord, Package> {
  let mut pkg_map = AHashMap::default();

  for (id, module_map) in mappings {
    for base in module_map.keys() {
      // Key is lodash, lodash/fp
      // `base` could be empty
      pkg_map.insert(
        {
          if base.is_empty() {
            JsWord::from(id.as_str())
          } else {
            JsWord::from(format!("{}/{}", &id, &base).as_str())
          }
        },
        Package::new(cwd, id, base).unwrap(),
      );
    }
  }

  pkg_map
}
