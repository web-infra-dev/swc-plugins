pub mod utils;

// reexports some same version libs
pub use anyhow;
pub use serde_json;
pub use swc;
pub use swc_atoms;
pub use swc_common;
pub use swc_ecma_ast;
pub use swc_ecma_codegen;
pub use swc_ecma_parser;
pub use swc_ecma_transforms;
pub use swc_ecma_transforms_base;
pub use swc_ecma_transforms_compat;
pub use swc_ecma_transforms_react;
pub use swc_ecma_transforms_typescript;
pub use swc_ecma_minifier;
pub use swc_ecma_visit;
pub use swc_ecma_preset_env;
pub use swc_ecma_transforms_testing;
pub use testing;
pub extern crate serde;
