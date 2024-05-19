//! A module for defining keyscope errors

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("provider not exists")]
    ProviderNotExists,

    #[error("validation not supported for provider")]
    ValidationNotSupported,

    #[error("empty arguments")]
    EmptyArguments,

    #[error("parameters not valid for provider")]
    KeyValidationParamsAreNotValid,
}
