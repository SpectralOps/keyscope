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

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub providers: Vec<providers::Provider>,
}

impl Default for Config {
    fn default() -> Self {
        let providers = vec![
            providers::tester(),
            providers::infura(),
            providers::covalenthq(),
            providers::asana(),
            providers::bitly(),
            providers::ipstack(),
            providers::localytics(),
            providers::algolia(),
            providers::branchio(),
            providers::browserstack(),
            providers::buildkite(),
            providers::datadog(),
            providers::github(),
            providers::github_ent(),
            providers::dropbox(),
            providers::gitlab(),
            providers::heroku(),
            providers::mailchimp(),
            providers::mailgun(),
            providers::pagerduty(),
            providers::circleci(),
            providers::facebook_access_token(),
            providers::salesforce(),
            providers::jumpcloud(),
            providers::saucelabs_us(),
            providers::saucelabs_eu(),
            providers::sendgrid(),
            providers::slack(),
            providers::slack_webhook(),
            providers::stripe(),
            providers::travisci(),
            providers::twilio(),
            providers::twitter(),
            providers::zendesk(),
            providers::firebase(),
            providers::aws(),
            providers::elastic_apm_secret(),
            providers::artifactory(),
            providers::ibm_cos(),
            providers::ibm_iam(),
            providers::ibm_cloudant(),
            providers::softlayer(),
            providers::square(),
            providers::telegram_bot(),
            providers::bingmaps(),
            providers::buttercms(),
            providers::wakatime(),
            providers::calendly(),
            providers::shodan(),
            providers::opsgenie(),
            providers::pendo(),
            providers::hubspot(),
            providers::lokalise(),
        ];
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
