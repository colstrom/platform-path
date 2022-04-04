use crate::{Error, Result};
use camino::Utf8PathBuf;
use directories::BaseDirs;
use std::path::PathBuf;
use structopt::StructOpt;
use strum::{Display, EnumIter, EnumProperty, EnumString};

#[derive(Debug, PartialEq, Eq, Hash, Display, StructOpt, EnumString, EnumIter, EnumProperty)]
#[cfg_attr(
  feature = "serde1",
  derive(serde::Serialize, serde::Deserialize),
  serde(rename_all = "kebab-case")
)]
#[strum(serialize_all = "kebab-case")]
#[structopt(
  rename_all = "kebab-case",
  about = "user-invisible standard directories"
)]
pub enum Base {
  #[structopt(about = "the user's cache directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Cache,
  #[structopt(about = "the user's config directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Config,
  #[structopt(about = "the user's data directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Data,
  #[structopt(about = "the user's local data directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  DataLocal,
  #[structopt(about = "the user's executable directory")]
  #[strum(props(linux = "supported", macos = "unsupported", windows = "unsupported"))]
  Executable,
  #[structopt(about = "the user's home directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Home,
  #[structopt(about = "the user's preference directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Preference,
  #[structopt(about = "the user's runtime directory")]
  #[strum(props(linux = "supported", macos = "unsupported", windows = "unsupported"))]
  Runtime,
  #[structopt(about = "the user's state directory")]
  #[strum(props(linux = "supported", macos = "unsupported", windows = "unsupported"))]
  State,
}

impl Base {
  pub fn dirs() -> Result<BaseDirs> {
    BaseDirs::new().ok_or(Error::InvalidHomeDirectory)
  }

  pub(crate) fn path_buf(&self) -> Result<PathBuf> {
    Self::dirs().and_then(|base| {
      match self {
        Self::Cache => Some(base.cache_dir()),
        Self::Config => Some(base.config_dir()),
        Self::Data => Some(base.data_dir()),
        Self::DataLocal => Some(base.data_local_dir()),
        Self::Executable => base.executable_dir(),
        Self::Home => Some(base.home_dir()),
        Self::Preference => Some(base.preference_dir()),
        Self::Runtime => base.runtime_dir(),
        Self::State => base.state_dir(),
      }
      .ok_or(Error::NotDefinedByPlatformStandard)
      .map(|path| path.to_path_buf())
    })
  }

  pub fn utf8_path_buf(&self) -> Result<Utf8PathBuf> {
    self
      .path_buf()
      .and_then(|path| Ok(Utf8PathBuf::try_from(path)?))
  }
}
