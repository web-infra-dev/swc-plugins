#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod transform {
    use core::pass::transform_pass;
    use std::sync::Arc;
    use crate::config::Config;
    use napi::{self, Env};
    use napi_derive::napi;
    use shared::{
        swc::{config::SourceMapsConfig, Compiler, TransformOutput},
        swc_common::{FileName, SourceMap},
        swc_ecma_ast::EsVersion,
        swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig},
        swc_ecma_visit::VisitMutWith,
    };
    pub struct Output {
        pub code: String,
        pub map: Option<String>,
    }
    impl napi::bindgen_prelude::TypeName for Output {
        fn type_name() -> &'static str {
            "Output"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Output {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Output,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { code, map } = val;
            obj.set("code", code)?;
            if map.is_some() {
                obj.set("map", map)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Output {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let code: String = obj.get("code")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"code")],
                    ));
                    res
                })
            })?;
            let map: Option<String> = obj.get("map")?;
            let val = Self { code, map };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Output {}
    impl From<TransformOutput> for Output {
        fn from(c: TransformOutput) -> Self {
            Self {
                code: c.code,
                map: c.map,
            }
        }
    }
    pub fn transform(env: Env, config: Config, code: String, map: Option<String>) -> Output {
        let source_filename = "test";
        let cm: Arc<SourceMap> = Arc::new(SourceMap::default());
        let input_map = map.map(|m| {
            shared::swc::sourcemap::SourceMap::from_slice(m.as_bytes())
                .expect("parse input sourcemap failed")
        });
        let fm = cm.new_source_file(FileName::Custom(source_filename.to_string()), code);
        let compiler = Compiler::new(cm);
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                dts: false,
                no_early_errors: false,
            }),
            EsVersion::Es2016,
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        let list_error = parser.take_errors();
        if list_error.iter().len() > 0 {
            let err_msg = list_error
                .iter()
                .map(|err| err.kind().msg())
                .collect::<Vec<_>>()
                .join("");
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&err_msg)],
                ));
            };
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["Lex scan failed"], &[]));
        }
        let module_result = parser.parse_module();
        if module_result.is_err() {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(
                        &module_result.err().unwrap().into_kind().msg().to_string(),
                    )],
                ));
            };
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["parse source failed"],
                &[],
            ));
        }
        let mut module = module_result.unwrap();
        ::core::panicking::panic("not implemented");
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    extern "C" fn __napi__transform(
        env: napi::bindgen_prelude::sys::napi_env,
        cb: napi::bindgen_prelude::sys::napi_callback_info,
    ) -> napi::bindgen_prelude::sys::napi_value {
        unsafe {
            napi::bindgen_prelude::CallbackInfo::<4usize>::new(env, cb, None)
                .and_then(|mut cb| {
                    let arg0 = {
                        <Config as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                            env,
                            cb.get_arg(0usize),
                        )?
                    };
                    let arg1 = {
                        <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                            env,
                            cb.get_arg(1usize),
                        )?
                    };
                    let arg2 = {
                        <Option<String> as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                            env,
                            cb.get_arg(2usize),
                        )?
                    };
                    napi::bindgen_prelude::within_runtime_if_available(move || {
                        let _ret =
                            { transform(napi::bindgen_prelude::Env::from(env), arg0, arg1, arg2) };
                        <Output as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
                    })
                })
                .unwrap_or_else(|e| {
                    napi::bindgen_prelude::JsError::from(e).throw_into(env);
                    std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                })
        }
    }
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    unsafe fn transform_js_function(
        env: napi::bindgen_prelude::sys::napi_env,
    ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
        let mut fn_ptr = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_create_function(
                env,
                "transform\0".as_ptr() as *const _,
                10usize,
                Some(__napi__transform),
                std::ptr::null_mut(),
                &mut fn_ptr,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => Err(::napi::Error::new(::napi::Status::from(c), {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Failed to register function `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"transform")],
                    ));
                    res
                })),
            }
        }?;
        napi::bindgen_prelude::register_js_function(
            "transform\0",
            transform_js_function,
            Some(__napi__transform),
        );
        Ok(fn_ptr)
    }
    #[allow(clippy::all)]
    #[allow(non_snake_case)]
    #[cfg(all(not(test), not(feature = "noop")))]
    extern "C" fn __napi_register__transform() {
        napi::bindgen_prelude::register_module_export(None, "transform\0", transform_js_function);
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __napi_register__transform___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn __napi_register__transform___rust_ctor___ctor() {
            __napi_register__transform()
        };
        __napi_register__transform___rust_ctor___ctor
    };
}
pub mod config {
    use napi::{self, Env, JsFunction};
    use napi_derive::napi;
    pub struct ImportPluginConfig {
        pub source: String,
        pub es: Option<ReplaceEs>,
        pub css: Option<ReplaceCss>,
    }
    impl napi::bindgen_prelude::TypeName for ImportPluginConfig {
        fn type_name() -> &'static str {
            "ImportPluginConfig"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for ImportPluginConfig {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: ImportPluginConfig,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { source, es, css } = val;
            obj.set("source", source)?;
            if es.is_some() {
                obj.set("es", es)?;
            }
            if css.is_some() {
                obj.set("css", css)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for ImportPluginConfig {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let source: String = obj.get("source")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"source")],
                    ));
                    res
                })
            })?;
            let es: Option<ReplaceEs> = obj.get("es")?;
            let css: Option<ReplaceCss> = obj.get("css")?;
            let val = Self { source, es, css };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for ImportPluginConfig {}
    pub struct ReplaceEs {
        pub replace: JsFunction,
        pub ignore: Option<Vec<String>>,
        pub lower: Option<bool>,
    }
    impl napi::bindgen_prelude::TypeName for ReplaceEs {
        fn type_name() -> &'static str {
            "ReplaceEs"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for ReplaceEs {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: ReplaceEs,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                replace,
                ignore,
                lower,
            } = val;
            obj.set("replace", replace)?;
            if ignore.is_some() {
                obj.set("ignore", ignore)?;
            }
            if lower.is_some() {
                obj.set("lower", lower)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for ReplaceEs {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let replace: JsFunction = obj.get("replace")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"replace")],
                    ));
                    res
                })
            })?;
            let ignore: Option<Vec<String>> = obj.get("ignore")?;
            let lower: Option<bool> = obj.get("lower")?;
            let val = Self {
                replace,
                ignore,
                lower,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for ReplaceEs {}
    pub struct ReplaceCss {
        pub replace: JsFunction,
        pub ignore: Option<Vec<String>>,
        pub lower: Option<bool>,
    }
    impl napi::bindgen_prelude::TypeName for ReplaceCss {
        fn type_name() -> &'static str {
            "ReplaceCss"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for ReplaceCss {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: ReplaceCss,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                replace,
                ignore,
                lower,
            } = val;
            obj.set("replace", replace)?;
            if ignore.is_some() {
                obj.set("ignore", ignore)?;
            }
            if lower.is_some() {
                obj.set("lower", lower)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for ReplaceCss {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let replace: JsFunction = obj.get("replace")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"replace")],
                    ));
                    res
                })
            })?;
            let ignore: Option<Vec<String>> = obj.get("ignore")?;
            let lower: Option<bool> = obj.get("lower")?;
            let val = Self {
                replace,
                ignore,
                lower,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for ReplaceCss {}
    pub struct ReactConfig {
        pub hmr: bool,
        pub auto_import_react: bool,
    }
    impl napi::bindgen_prelude::TypeName for ReactConfig {
        fn type_name() -> &'static str {
            "ReactConfig"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for ReactConfig {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: ReactConfig,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                hmr,
                auto_import_react,
            } = val;
            obj.set("hmr", hmr)?;
            obj.set("autoImportReact", auto_import_react)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for ReactConfig {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let hmr: bool = obj.get("hmr")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"hmr")],
                    ));
                    res
                })
            })?;
            let auto_import_react: bool = obj.get("autoImportReact")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"autoImportReact")],
                    ));
                    res
                })
            })?;
            let val = Self {
                hmr,
                auto_import_react,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for ReactConfig {}
    pub struct Config {
        pub ts: bool,
        pub react: PresetReact,
        /// similar to babel-preset-env
        /// the way it looks up for browserlist is the same as modern.js
        pub envii: PresetEnv,
        /// Internal rust-swc Plugins
        pub pluginImport: Vec<PluginImportItem>,
    }
    impl napi::bindgen_prelude::TypeName for Config {
        fn type_name() -> &'static str {
            "Config"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Config {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Config,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                ts,
                react,
                envii,
                pluginImport,
            } = val;
            obj.set("ts", ts)?;
            obj.set("react", react)?;
            obj.set("envii", envii)?;
            obj.set("pluginImport", pluginImport)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Config {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let ts: bool = obj.get("ts")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"ts")],
                    ));
                    res
                })
            })?;
            let react: PresetReact = obj.get("react")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"react")],
                    ));
                    res
                })
            })?;
            let envii: PresetEnv = obj.get("envii")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"envii")],
                    ));
                    res
                })
            })?;
            let pluginImport: Vec<PluginImportItem> =
                obj.get("pluginImport")?.ok_or_else(|| {
                    napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Missing field `", "`"],
                            &[::core::fmt::ArgumentV1::new_display(&"pluginImport")],
                        ));
                        res
                    })
                })?;
            let val = Self {
                ts,
                react,
                envii,
                pluginImport,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Config {}
    pub struct _Config {
        pub env: i32,
        pub val: bool,
    }
    impl napi::bindgen_prelude::TypeName for _Config {
        fn type_name() -> &'static str {
            "_Config"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for _Config {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: _Config,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { env, val } = val;
            obj.set("env", env)?;
            obj.set("val", val)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for _Config {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let env: i32 = obj.get("env")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"env")],
                    ));
                    res
                })
            })?;
            let val: bool = obj.get("val")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"val")],
                    ));
                    res
                })
            })?;
            let val = Self { env, val };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for _Config {}
    pub struct PluginImportItem {
        pub source: String,
        pub transform: String,
    }
    impl napi::bindgen_prelude::TypeName for PluginImportItem {
        fn type_name() -> &'static str {
            "PluginImportItem"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for PluginImportItem {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: PluginImportItem,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { source, transform } = val;
            obj.set("source", source)?;
            obj.set("transform", transform)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for PluginImportItem {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let source: String = obj.get("source")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"source")],
                    ));
                    res
                })
            })?;
            let transform: String = obj.get("transform")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"transform")],
                    ));
                    res
                })
            })?;
            let val = Self { source, transform };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for PluginImportItem {}
    pub enum ReactMode {
        Classic,
        Automatic,
    }
    impl From<String> for ReactMode {
        fn from(s: String) -> Self {
            match s.as_str() {
                "classic" => ReactMode::Classic,
                "automatic" => ReactMode::Automatic,
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["config.react.runtime invalid"],
                    &[],
                )),
            }
        }
    }
    pub struct PresetReact {
        pub runtime: String,
        pub development: bool,
        /// default: NODE_ENV === 'development'
        pub throw_if_namespace: bool,
        /// default: false
        /// React automatic runtime
        pub import_source: String,
        /// React classic runtime
        pub pragma: String,
        pub pragma_frag: String,
        /// HMR react-refresh
        pub hmr: bool,
    }
    impl napi::bindgen_prelude::TypeName for PresetReact {
        fn type_name() -> &'static str {
            "PresetReact"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for PresetReact {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: PresetReact,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                runtime,
                development,
                throw_if_namespace,
                import_source,
                pragma,
                pragma_frag,
                hmr,
            } = val;
            obj.set("runtime", runtime)?;
            obj.set("development", development)?;
            obj.set("throwIfNamespace", throw_if_namespace)?;
            obj.set("importSource", import_source)?;
            obj.set("pragma", pragma)?;
            obj.set("pragmaFrag", pragma_frag)?;
            obj.set("hmr", hmr)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for PresetReact {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let runtime: String = obj.get("runtime")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"runtime")],
                    ));
                    res
                })
            })?;
            let development: bool = obj.get("development")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"development")],
                    ));
                    res
                })
            })?;
            let throw_if_namespace: bool = obj.get("throwIfNamespace")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"throwIfNamespace")],
                    ));
                    res
                })
            })?;
            let import_source: String = obj.get("importSource")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"importSource")],
                    ));
                    res
                })
            })?;
            let pragma: String = obj.get("pragma")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"pragma")],
                    ));
                    res
                })
            })?;
            let pragma_frag: String = obj.get("pragmaFrag")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"pragmaFrag")],
                    ));
                    res
                })
            })?;
            let hmr: bool = obj.get("hmr")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"hmr")],
                    ));
                    res
                })
            })?;
            let val = Self {
                runtime,
                development,
                throw_if_namespace,
                import_source,
                pragma,
                pragma_frag,
                hmr,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for PresetReact {}
    pub enum PolyfillMode {
        Entry,
        Usage,
        None,
    }
    impl From<String> for PolyfillMode {
        fn from(s: String) -> Self {
            match s.as_str() {
                "entry" => PolyfillMode::Entry,
                "usage" => PolyfillMode::Usage,
                "false" => PolyfillMode::None,
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid mode"],
                    &[],
                )),
            }
        }
    }
    pub struct PresetEnv {
        pub targets: String,
        pub core_js: String,
        pub mode: String,
    }
    impl napi::bindgen_prelude::TypeName for PresetEnv {
        fn type_name() -> &'static str {
            "PresetEnv"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for PresetEnv {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: PresetEnv,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                targets,
                core_js,
                mode,
            } = val;
            obj.set("targets", targets)?;
            obj.set("coreJs", core_js)?;
            obj.set("mode", mode)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for PresetEnv {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let targets: String = obj.get("targets")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"targets")],
                    ));
                    res
                })
            })?;
            let core_js: String = obj.get("coreJs")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"coreJs")],
                    ));
                    res
                })
            })?;
            let mode: String = obj.get("mode")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"mode")],
                    ));
                    res
                })
            })?;
            let val = Self {
                targets,
                core_js,
                mode,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for PresetEnv {}
    pub struct Plugins {
        pub import_plugin: Option<Vec<ImportPluginConfig>>,
        pub react: Option<ReactConfig>,
    }
    impl napi::bindgen_prelude::TypeName for Plugins {
        fn type_name() -> &'static str {
            "Plugins"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Plugins {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Plugins,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                import_plugin,
                react,
            } = val;
            if import_plugin.is_some() {
                obj.set("importPlugin", import_plugin)?;
            }
            if react.is_some() {
                obj.set("react", react)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Plugins {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let import_plugin: Option<Vec<ImportPluginConfig>> = obj.get("importPlugin")?;
            let react: Option<ReactConfig> = obj.get("react")?;
            let val = Self {
                import_plugin,
                react,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Plugins {}
    fn gen_replace(env: Env, f: JsFunction) -> Box<dyn Fn(String) -> String> {
        Box::new(move |s: String| -> String {
            f.call(None, &[env.create_string(&s).unwrap()])
                .unwrap()
                .coerce_to_string()
                .unwrap()
                .into_utf8()
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        })
    }
}
pub mod minify {
    use napi_derive::napi;
    use shared::{
        serde_json,
        swc::{config::SourceMapsConfig, Compiler, TransformOutput},
        swc_common::{FileName, Mark, SourceMap},
        swc_ecma_ast::{EsVersion, Program},
        swc_ecma_minifier::{
            optimize,
            option::{ExtraOptions, MinifyOptions},
        },
        swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig},
    };
    use std::sync::Arc;
    pub struct Output {
        pub code: String,
        pub map: Option<String>,
    }
    impl napi::bindgen_prelude::TypeName for Output {
        fn type_name() -> &'static str {
            "Output"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Output {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Output,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { code, map } = val;
            obj.set("code", code)?;
            if map.is_some() {
                obj.set("map", map)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Output {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let code: String = obj.get("code")?.ok_or_else(|| {
                napi::bindgen_prelude::Error::new(napi::bindgen_prelude::Status::InvalidArg, {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing field `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"code")],
                    ));
                    res
                })
            })?;
            let map: Option<String> = obj.get("map")?;
            let val = Self { code, map };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Output {}
    impl From<TransformOutput> for Output {
        fn from(c: TransformOutput) -> Self {
            Self {
                code: c.code,
                map: c.map,
            }
        }
    }
    pub fn minify(config: String, code: String, map: Option<String>) -> Output {
        let config: MinifyOptions =
            serde_json::from_str(config.as_str()).expect("Minify options invalid");
        let source_filename = "test";
        let cm: Arc<SourceMap> = Arc::new(SourceMap::default());
        let input_map = map.map(|m| {
            shared::swc::sourcemap::SourceMap::from_slice(m.as_bytes())
                .expect("parse input sourcemap failed")
        });
        let fm = cm.new_source_file(FileName::Custom(source_filename.to_string()), code);
        let compiler = Compiler::new(cm.clone());
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                dts: false,
                no_early_errors: false,
            }),
            EsVersion::Es2016,
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        let list_error = parser.take_errors();
        if list_error.iter().len() > 0 {
            let err_msg = list_error
                .iter()
                .map(|err| err.kind().msg())
                .collect::<Vec<_>>()
                .join("");
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&err_msg)],
                ));
            };
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["Lex scan failed"], &[]));
        }
        let module_result = parser.parse_module();
        if module_result.is_err() {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(
                        &module_result.err().unwrap().into_kind().msg().to_string(),
                    )],
                ));
            };
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["parse source failed"],
                &[],
            ));
        }
        let module = module_result.unwrap();
        let module = optimize(
            Program::Module(module),
            cm,
            None,
            None,
            &config,
            &ExtraOptions {
                unresolved_mark: Mark::new(),
                top_level_mark: Mark::new(),
            },
        );
        compiler
            .print(
                &module,
                Some(source_filename),
                None,
                false,
                EsVersion::Es5,
                SourceMapsConfig::Bool(true),
                &Default::default(),
                input_map.as_ref(),
                true,
                None,
                true,
                false,
            )
            .map(|output| output.into())
            .unwrap()
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    extern "C" fn __napi__minify(
        env: napi::bindgen_prelude::sys::napi_env,
        cb: napi::bindgen_prelude::sys::napi_callback_info,
    ) -> napi::bindgen_prelude::sys::napi_value {
        unsafe {
            napi::bindgen_prelude::CallbackInfo::<3usize>::new(env, cb, None)
                .and_then(|mut cb| {
                    let arg0 = {
                        <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                            env,
                            cb.get_arg(0usize),
                        )?
                    };
                    let arg1 = {
                        <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                            env,
                            cb.get_arg(1usize),
                        )?
                    };
                    let arg2 = {
                        <Option<String> as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                            env,
                            cb.get_arg(2usize),
                        )?
                    };
                    napi::bindgen_prelude::within_runtime_if_available(move || {
                        let _ret = { minify(arg0, arg1, arg2) };
                        <Output as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
                    })
                })
                .unwrap_or_else(|e| {
                    napi::bindgen_prelude::JsError::from(e).throw_into(env);
                    std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                })
        }
    }
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    unsafe fn minify_js_function(
        env: napi::bindgen_prelude::sys::napi_env,
    ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
        let mut fn_ptr = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_create_function(
                env,
                "minify\0".as_ptr() as *const _,
                7usize,
                Some(__napi__minify),
                std::ptr::null_mut(),
                &mut fn_ptr,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => Err(::napi::Error::new(::napi::Status::from(c), {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Failed to register function `", "`"],
                        &[::core::fmt::ArgumentV1::new_display(&"minify")],
                    ));
                    res
                })),
            }
        }?;
        napi::bindgen_prelude::register_js_function(
            "minify\0",
            minify_js_function,
            Some(__napi__minify),
        );
        Ok(fn_ptr)
    }
    #[allow(clippy::all)]
    #[allow(non_snake_case)]
    #[cfg(all(not(test), not(feature = "noop")))]
    extern "C" fn __napi_register__minify() {
        napi::bindgen_prelude::register_module_export(None, "minify\0", minify_js_function);
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __napi_register__minify___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn __napi_register__minify___rust_ctor___ctor() {
            __napi_register__minify()
        };
        __napi_register__minify___rust_ctor___ctor
    };
}
