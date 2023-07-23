/*
 * The following code is modified based on
 * https://github.com/swc-project/plugins/tree/main/packages/loadable-components.
 * As we need this plugin not enable ecma_plugin_transform feature of swc_core
 *
 * Copyright (c) 2021 kdy1(Donny/강동윤), kwonoj(OJ Kwon), XiNiHa(Cosmo Shin (신의하)), beaumontjonathan(Jonathan Beaumont)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![allow(clippy::all)]

use swc_core::ecma::ast::*;

pub(crate) fn get_import_arg(call: &CallExpr) -> &Expr {
  &call.args[0].expr
}
