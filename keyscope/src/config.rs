use std::collections::HashMap;

use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use crate::{providers, Error, Result};

lazy_static! {
    pub static ref DEFAULT_CONFIG: Definitions =
        serde_yaml::from_str(include_str!("config.yaml")).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    pub providers: HashMap<String, providers::ActionMapping>,
}

impl From<Definitions> for Config {
    fn from(definitions: Definitions) -> Self {
        let providers = definitions
            .providers
            .into_iter()
            .map(|(name, config)| providers::Provider::new(name, config))
            .collect();

        Self { providers }
    }
}

#[derive(Debug, Serialize)]
pub struct Config {
    pub providers: Vec<providers::Provider>,
}

impl Default for Config {
    fn default() -> Self {
        let providers: Vec<providers::Provider> = providers::all_providers();
        Self { providers }
    }
}

impl Config {
    /// Get provider by name
    ///
    /// # Errors
    ///
    /// Return [`Error`] if provider not exists
    pub fn get_providers(&self, name: &str) -> Result<&providers::Provider> {
        self.providers
            .iter()
            .find(|p| p.name() == name)
            .ok_or(Error::ProviderNotExists)
    }
}
