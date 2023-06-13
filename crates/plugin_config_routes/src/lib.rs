#![feature(let_chains)]
use swc_core::{
  ecma::{
    ast::{Expr, KeyValueProp, Prop, PropName, PropOrSpread},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
  },
  quote,
};
#[derive(Default, Debug, serde::Deserialize)]
pub struct ConfigRoutesConfig {
  pub lazy: Option<bool>,
}

pub fn plugin_config_routes(config: &ConfigRoutesConfig) -> impl Fold {
  as_folder(ConfigRoutes {
    lazy: config.lazy.unwrap_or_default(),
  })
}

fn find_target_prop_from_props<'a>(
  props: &'a Vec<PropOrSpread>,
  target: &'static str,
) -> Option<&'a KeyValueProp> {
  props.iter().find_map(|prop_spread| {
    let prop = prop_spread.as_prop()?.as_key_value()?;
    let key = &prop.key;
    if key
      .as_str()
      .map(|key| key.value.eq(target))
      .or(key.as_ident().map(|key| key.sym.eq(target)))?
    {
      Some(prop)
    } else {
      None
    }
  })
}

fn get_target_prop<'a>(prop: &'a mut Prop, target: &'static str) -> Option<&'a mut KeyValueProp> {
  let prop = prop.as_mut_key_value()?;
  let key = &prop.key;
  if key
    .as_str()
    .map(|key| key.value.eq(target))
    .or(key.as_ident().map(|key| key.sym.eq(target)))?
  {
    Some(prop)
  } else {
    None
  }
}

struct ConfigRoutes {
  lazy: bool,
}
impl VisitMut for ConfigRoutes {
  noop_visit_mut_type!();

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    fn inner_visit_mut_expr(lazy: bool, expr: &mut Expr) -> Option<()> {
      if lazy {
        return None;
      }
      let object_expr = expr.as_mut_object()?;

      let component_prop = find_target_prop_from_props(&object_expr.props, "component")?;

      object_expr
        .props
        .push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Str("module".into()),
          value: component_prop.value.clone(),
        }))));

      Some(())
    }

    inner_visit_mut_expr(self.lazy, expr);

    expr.visit_mut_children_with(self);
  }

  fn visit_mut_prop(&mut self, prop: &mut Prop) {
    if !self.lazy && let Some(module) = get_target_prop(prop, "module")  {
      module.value = Box::new(quote!("require($value)" as Expr, value: Expr = module.value.as_ref().clone()));  
    }

    if let Some(component) = get_target_prop(prop, "component") {
      if self.lazy {
        component.value = Box::new(quote!(
          "loadable(() => import($value))" as Expr,
          value: Expr = component.value.as_ref().clone()
        ));
      } else {
        component.value = Box::new(quote!(
          "require($value).default" as Expr,
          value: Expr = component.value.as_ref().clone()
        ))
      }
    }

    prop.visit_mut_children_with(self);
  }
}
