//! A module for exporting content
//!
//! This module provides functionality to export content to different formats
//! and destinations.
use console::style;
use keyscope::providers::Provider;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use service_policy_kit::data::Param;

#[derive(clap::ValueEnum, Debug, Clone, Deserialize, Serialize)]
pub enum Reporter {
    /// Export to the console STDOUT.
    #[serde(rename = "console")]
    Stdout,
    /// Json format.
    #[serde(rename = "json")]
    Json,
    /// Junit format.
    #[serde(rename = "junit")]
    Junit,
    /// disable check results.
    #[serde(rename = "quiet")]
    Quiet,
}

impl std::fmt::Display for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_variant_name(self).expect("only enum supported").fmt(f)
    }
}

pub const fn provider_not_found() -> &'static str {
    "params not found `keyscope providers` to see all available providers"
}

pub fn requirements_results(provider: &Provider, params: &[Param]) -> String {
    let params_str: String = params
        .iter()
        .map(|p| {
            format!(
                " - param: {}\n   desc: {}\n   optional: {}",
                p.name, p.desc, p.optional
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "provider {} requires:\n{params_str}\n\nTo use from the CLI, run:\n  keyscope validate {} \
         -p PARAM1 -p PARAM2 ...",
        provider.name(),
        provider.name()
    )
}

pub fn supported_providers(providers: &[Provider]) -> String {
    let mut result = String::new();

    let mut providers = providers.iter().collect::<Vec<_>>();
    providers.sort_by(|a, b| a.name().cmp(b.name()));

    for provider in &providers {
        if let Some(validation) = provider.config().validation.as_ref() {
            result.push_str(&format!(
                "{}\nkeyscope validate {} -p {}\n\n",
                style(validation.request.get_id()).magenta(),
                style(provider.name()).yellow(),
                style(
                    validation
                        .request
                        .params
                        .as_ref()
                        .map(|p| p
                            .iter()
                            .map(|param| style(param.name.clone()).blue().to_string())
                            .collect::<Vec<_>>()
                            .join(" -p "))
                        .unwrap_or_default()
                )
                .blue()
            ));
        }
    }
    result.push_str(&format!(
        "Total {} providers available.",
        providers
            .iter()
            .filter(|provider| provider.config().validation.is_some())
            .count(),
    ));
    result
}
