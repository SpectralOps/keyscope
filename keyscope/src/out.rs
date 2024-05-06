//! A module for exporting content
//!
//! This module provides functionality to export content to different formats
//! and destinations.
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

#[derive(clap::ValueEnum, Clone, Deserialize, Serialize)]
pub enum Reporter {
    /// Export to the console STDOUT.
    #[serde(alias = "console")]
    STDOUT,
}

impl std::fmt::Display for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_variant_name(self).expect("only enum supported").fmt(f)
    }
}
