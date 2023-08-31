use swc_core::{
  ecma::{ast::Program, visit::VisitMutWith},
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn transform(mut program: Program, meta: TransformPluginProgramMetadata) -> Program {
  let config = meta.get_transform_plugin_config();

  let config = config
    .map(|raw| {
      serde_json::from_str(&raw).expect("Failed to parse config of react-const-elements")
    })
    .unwrap_or_default();

  let mut v = plugin_react_const_elements::react_const_elements(config);

  program.visit_mut_with(&mut v);

  program
}
