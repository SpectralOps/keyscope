use serde_derive::{Deserialize, Serialize};
use service_policy_kit::data::{Context, Interaction, Param};
use service_policy_kit::runner::{RunOptions, RunnerReport, SequenceRunner};

use crate::{config::DEFAULT_CONFIG, Error, Result};

pub trait ProviderTrait: Sync {
    fn name(&self) -> &str;
    fn config(&self) -> &ActionMapping;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    name: String,
    config: ActionMapping,
}

impl Provider {
    /// Create new provider
    #[must_use]
    pub const fn new(name: String, config: ActionMapping) -> Self {
        Self { name, config }
    }

    /// Get provider name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get provider configuration
    #[must_use]
    pub const fn config(&self) -> &ActionMapping {
        &self.config
    }

    /// Get parameters fields from request configuration
    ///
    /// # Errors
    pub fn get_validation_request_params(&self) -> Result<Option<&Vec<Param>>> {
        Ok(self
            .config
            .validation
            .as_ref()
            .ok_or(Error::ValidationNotSupported)?
            .request
            .params
            .as_ref())
    }

    /// Validate context parameters fields to the providers
    ///
    /// # Errors
    /// Return an [`Error`] when validation field is None or the parameters is invalid
    pub fn is_valid_params_for_key_validate(&self, context: &Context) -> Result<()> {
        self.config
            .validation
            .as_ref()
            .ok_or(Error::ValidationNotSupported)?
            .ensure_requirements(context)
            .or(Err(Error::KeyValidationParamsAreNotValid))
    }

    /// ...
    ///
    /// # Errors
    ///
    /// Return an [`Error`] with the validation process
    pub fn key_validate<T>(&self, params: &[T]) -> Result<RunnerReport>
    where
        T: AsRef<str>,
    {
        let opts = RunOptions::build(None, false, None, false);
        self.key_validate_with_opts(params, &opts)
    }

    /// ...
    ///
    /// # Errors
    ///
    /// Return an [`Error`] with the validation process
    pub fn key_validate_with_opts<T>(&self, params: &[T], opts: &RunOptions) -> Result<RunnerReport>
    where
        T: AsRef<str>,
    {
        if params.is_empty() {
            return Err(Error::EmptyArguments);
        }

        let interaction = self
            .config
            .validation
            .as_ref()
            .ok_or(Error::ValidationNotSupported)?;

        let mut context = Context::new();

        params
            .iter()
            .enumerate()
            .map(|(i, value)| {
                (
                    format!("{}_{}", self.name(), i + 1),
                    value.as_ref().to_string(),
                )
            })
            .for_each(|(k, v)| {
                context.vars_bag.insert(k, v);
            });

        self.is_valid_params_for_key_validate(&context)?;

        Ok(SequenceRunner::from_opts(opts).run(&mut context, &vec![interaction.clone()]))
    }
}

macro_rules! define_provider_type {
    ($(($struct_name:ident, $key:expr)),*) => {
        $(
            lazy_static::lazy_static! {
                pub static ref $struct_name: Provider = Provider::new($key.to_string(), DEFAULT_CONFIG.providers.get($key).unwrap().clone());
            }
        )*

        #[must_use] pub fn all_providers_ref() -> Vec<&'static Provider> {
            vec![$(&$struct_name),*]
        }

        #[must_use] pub fn all_providers() -> Vec<Provider> {
            vec![$($struct_name.clone()),*]
        }
    };
}

// Example usage
define_provider_type!(
    (TESTER, "tester"),
    (INFURA, "infura"),
    (COVALENTHQ, "covalenthq"),
    (ASANA, "asana"),
    (BITLY, "bitly"),
    (IPSTACK, "ipstack"),
    (LOCALYTICS, "localytics"),
    (ALGOLIA, "algolia"),
    (BRANCHIO, "branchio"),
    (BROWSERSTACK, "browserstack"),
    (BUILDKITE, "buildkite"),
    (DATADOG, "datadog"),
    (GITHUB, "github"),
    (GITHUB_ENT, "github-ent"),
    (DROPBOX, "dropbox"),
    (GITLAB, "gitlab"),
    (HEROKU, "heroku"),
    (MAILCHIMP, "mailchimp"),
    (MAILGUN, "mailgun"),
    (PAGERDUTY, "pagerduty"),
    (CIRCLECI, "circleci"),
    (FACEBOOK_ACCESS_TOKEN, "facebook-access-token"),
    (SALESFORCE, "salesforce"),
    (JUMPCLOUD, "jumpcloud"),
    (SAUCELABS_US, "saucelabs-us"),
    (SAUCELABS_EU, "saucelabs-eu"),
    (SENDGRID, "sendgrid"),
    (SLACK, "slack"),
    (SLACK_WEBHOOK, "slack-webhook"),
    (STRIPE, "stripe"),
    (TRAVISCI, "travisci"),
    (TWILIO, "twilio"),
    (TWITTER, "twitter"),
    (ZENDESK, "zendesk"),
    (FIREBASE, "firebase"),
    (AWS, "aws"),
    (ELASTIC_APM_SECRET, "elastic-apm-secret"),
    (ARTIFACTORY, "artifactory"),
    (IBM_COS, "ibm-cos"),
    (IBM_IAM, "ibm-iam"),
    (IBM_CLOUDANT, "ibm-cloudant"),
    (SOFTLAYER, "softlayer"),
    (SQUARE, "square"),
    (TELEGRAM_BOT, "telegram-bot"),
    (BINGMAPS, "bingmaps"),
    (BUTTERCMS, "buttercms"),
    (WAKATIME, "wakatime"),
    (CALENDLY, "calendly"),
    (SHODAN, "shodan"),
    (OPSGENIE, "opsgenie"),
    (PENDO, "pendo"),
    (HUBSPOT, "hubspot"),
    (LOKALISE, "lokalise")
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMapping {
    pub validation: Option<Interaction>,
}
