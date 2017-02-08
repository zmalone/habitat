// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;
use std::string::ToString;

use hcore::package::{PackageIdent, PackageInstall, Identifiable};
use manager::service::config::ServiceConfig;
use serde_json;
use toml;

use handlebars::{Context, Handlebars, Helper, HelperDef, Renderable, RenderError, RenderContext,
                 Template};

type RenderResult = Result<(), RenderError>;

#[derive(Clone, Copy)]
pub struct EachAliveHelper;

impl HelperDef for EachAliveHelper {
    fn call(&self, h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        debug!("received: {}", rc.context().data());
        match serde_json::from_value::<ServiceConfig>(rc.context().data().clone()) {
            Ok(data) => {
                debug!("data = {:?}", data);
            }
            Err(_) => {
                debug!("Unable to deserialize ServiceConfig.");
            }

        };
        h.template().map(|t| t.render(r, rc)).unwrap_or(Ok(()))
    }
}