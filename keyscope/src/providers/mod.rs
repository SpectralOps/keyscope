use serde_derive::{Deserialize, Serialize};
use service_policy_kit::data::{Context, Interaction, Param};
use service_policy_kit::runner::{RunOptions, RunnerReport, SequenceRunner};

use crate::{config::DEFAULT_CONFIG, Error, Result};

pub trait ProviderTrait: Sync {
    fn name(&self) -> &str;
    fn config(&self) -> &ActionMapping;
}

#[derive(Debug, Serialize, Deserialize)]
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
            #[must_use]
            pub fn $struct_name() -> Provider {
                Provider::new(stringify!($struct_name).to_string(), DEFAULT_CONFIG.providers.get($key).unwrap().clone())
            }
        )*
    };
}

// Example usage
define_provider_type!(
    (tester, "tester"),
    (infura, "infura"),
    (covalenthq, "covalenthq"),
    (asana, "asana"),
    (bitly, "bitly"),
    (ipstack, "ipstack"),
    (localytics, "localytics"),
    (algolia, "algolia"),
    (branchio, "branchio"),
    (browserstack, "browserstack"),
    (buildkite, "buildkite"),
    (datadog, "datadog"),
    (github, "github"),
    (github_ent, "github-ent"),
    (dropbox, "dropbox"),
    (gitlab, "gitlab"),
    (heroku, "heroku"),
    (mailchimp, "mailchimp"),
    (mailgun, "mailgun"),
    (pagerduty, "pagerduty"),
    (circleci, "circleci"),
    (facebook_access_token, "facebook-access-token"),
    (salesforce, "salesforce"),
    (jumpcloud, "jumpcloud"),
    (saucelabs_us, "saucelabs-us"),
    (saucelabs_eu, "saucelabs-eu"),
    (sendgrid, "sendgrid"),
    (slack, "slack"),
    (slack_webhook, "slack-webhook"),
    (stripe, "stripe"),
    (travisci, "travisci"),
    (twilio, "twilio"),
    (twitter, "twitter"),
    (zendesk, "zendesk"),
    (firebase, "firebase"),
    (aws, "aws"),
    (elastic_apm_secret, "elastic-apm-secret"),
    (artifactory, "artifactory"),
    (ibm_cos, "ibm-cos"),
    (ibm_iam, "ibm-iam"),
    (ibm_cloudant, "ibm-cloudant"),
    (softlayer, "softlayer"),
    (square, "square"),
    (telegram_bot, "telegram-bot"),
    (bingmaps, "bingmaps"),
    (buttercms, "buttercms"),
    (wakatime, "wakatime"),
    (calendly, "calendly"),
    (shodan, "shodan"),
    (opsgenie, "opsgenie"),
    (pendo, "pendo"),
    (hubspot, "hubspot"),
    (lokalise, "lokalise")
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMapping {
    pub validation: Option<Interaction>,
}
