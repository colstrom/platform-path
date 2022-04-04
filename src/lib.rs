mod command;
mod output;
mod platform;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  #[error("invalid home directory")]
  InvalidHomeDirectory,
  #[error("platform standard does not define requested directory")]
  NotDefinedByPlatformStandard,
  #[error("path contains invalid unicode: {0}")]
  InvalidUnicode(#[from] camino::FromPathBufError),
  #[cfg(feature = "json")]
  #[error("json error: {0}")]
  JsonError(#[from] serde_json::Error),
  #[cfg(feature = "yaml")]
  #[error("yaml error: {0}")]
  YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub use command::Command;
pub use platform::{Base, Project, ProjectOptions, User};
