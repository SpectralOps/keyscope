use anyhow::anyhow;
use anyhow::Result as AnyResult;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use service_policy_kit::data::{Context, Interaction, Param};

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    pub providers: HashMap<String, ActionMapping>,
}
impl Definitions {
    pub fn requirements_for(&self, provider: &str) -> AnyResult<Option<Vec<Param>>> {
        match self.providers.get(provider) {
            Some(p) => match p.validation.as_ref() {
                Some(validation) => Ok(validation.request.params.clone()),
                None => Err(anyhow!("no validation for provider {}", provider)),
            },
            None => Err(anyhow!("cannot find provider {}", provider)),
        }
    }
    pub fn validation_for(&self, context: &Context, provider: &str) -> AnyResult<Interaction> {
        match self.providers.get(provider) {
            Some(p) => match p.validation.as_ref() {
                Some(validation) => {
                    validation.ensure_requirements(context)?;
                    Ok(validation.clone())
                }
                None => Err(anyhow!("no validation for provider {}", provider)),
            },
            None => Err(anyhow!("cannot find provider {}", provider)),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMapping {
    pub validation: Option<Interaction>,
}

pub const BANNER: &str = r#"
    __                                       
   / /_____  __  ________________  ____  ___ 
  / // / _ \/ / / / ___/ ___/ __ \/ __ \/ _ \
 /   </  __/ /_/ (__  ) /__/ /_/ / /_/ /  __/
/_/|_|\___/\__, /____/\___/\____/ .___/\___/ 
          (____/               /_/"#;

pub const DEFS: &str = include_str!("defs.yaml");
