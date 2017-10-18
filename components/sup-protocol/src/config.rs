// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use toml;

use package::Pkg;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Cfg {
    /// Default level configuration loaded by a Package's `default.toml`
    pub default: Option<toml::Value>,
    /// User level configuration loaded by a Service's `user.toml`
    pub user: Option<toml::Value>,
    /// Gossip level configuration loaded by a census group
    pub gossip: Option<toml::Value>,
    /// Environment level configuration loaded by the Supervisor's process environment
    pub environment: Option<toml::Value>,
    /// Last known incarnation number of the census group's service config
    pub gossip_incarnation: u64,
}

impl Cfg {
    pub fn new<T>(pkg: &Pkg, config_from: Option<T>) -> Result<Cfg>
    where
        T: AsRef<Path>,
    {
        let pkg_root = config_from.map(|m| m.as_ref()).unwrap_or(&pkg.path);
        let mut cfg = Cfg::default();
        cfg.load_default(&pkg_root)?;
        cfg.load_user(&pkg)?;
        cfg.load_environment(&pkg)?;
        Ok(cfg)
    }

    /// Returns a subset of the overall configuration whitelisted by the given package's exports.
    pub fn to_exported(&self, pkg: &Pkg) -> Result<toml::value::Table> {
        let mut map = toml::value::Table::default();
        let cfg = toml::Value::try_from(&self).unwrap();
        for (key, path) in pkg.exports.iter() {
            let fields: Vec<&str> = path.split('.').collect();
            let mut curr = &cfg;
            let mut found = false;

            // JW TODO: the TOML library only provides us with a
            // function to retrieve a value with a path which returns a
            // reference. We actually want the value for ourselves.
            // Let's improve this later to avoid allocating twice.
            for field in fields {
                match curr.get(field) {
                    Some(val) => {
                        curr = val;
                        found = true;
                    }
                    None => found = false,
                }
            }

            if found {
                map.insert(key.clone(), curr.clone());
            }
        }
        Ok(map)
    }

    fn load_default<T>(&mut self, config_from: T) -> Result<()>
    where
        T: AsRef<Path>,
    {
        let path = config_from.as_ref().join("default.toml");
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open 'default.toml', {}, {}", path.display(), e);
                self.default = None;
                return Ok(());
            }
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {
                let toml = toml::de::from_str(&config).map_err(|e| {
                    sup_error!(Error::TomlParser(e))
                })?;
                self.default = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                warn!("Failed to read 'default.toml', {}, {}", path.display(), e);
                self.default = None;
            }
        }
        Ok(())
    }

    fn load_user(&mut self, pkg: &Pkg) -> Result<()> {
        let path = pkg.svc_path.join("user.toml");
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open 'user.toml', {}, {}", path.display(), e);
                self.user = None;
                return Ok(());
            }
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {
                let toml = toml::de::from_str(&config).map_err(|e| {
                    sup_error!(Error::TomlParser(e))
                })?;
                self.user = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                warn!("Failed to load 'user.toml', {}, {}", path.display(), e);
                self.user = None;
            }
        }
        Ok(())
    }

    fn load_environment(&mut self, pkg: &Pkg) -> Result<()> {
        let var_name = format!("{}_{}", ENV_VAR_PREFIX, pkg.name)
            .to_ascii_uppercase()
            .replace("-", "_");
        match env::var(&var_name) {
            Ok(config) => {
                match toml::de::from_str(&config) {
                    Ok(toml) => {
                        self.environment = Some(toml::Value::Table(toml));
                        return Ok(());
                    }
                    Err(err) => debug!("Attempted to parse env config as toml and failed {}", err),
                }
                match serde_json::from_str(&config) {
                    Ok(json) => {
                        self.environment = Some(toml::Value::Table(json));
                        return Ok(());
                    }
                    Err(err) => debug!("Attempted to parse env config as json and failed {}", err),
                }
                self.environment = None;
                Err(sup_error!(Error::BadEnvConfig(var_name)))
            }
            Err(e) => {
                debug!(
                    "Looking up environment variable {} failed: {:?}",
                    var_name,
                    e
                );
                self.environment = None;
                Ok(())
            }
        }
    }
}
