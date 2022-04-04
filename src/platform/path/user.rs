use crate::{Error, Result};
use camino::Utf8PathBuf;
use directories::UserDirs;
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
#[structopt(rename_all = "kebab-case", about = "user-facing standard directories")]
pub enum User {
  #[structopt(about = "the user's audio directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Audio,
  #[structopt(about = "the user's desktop directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Desktop,
  #[structopt(about = "the user's document directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Document,
  #[structopt(about = "the user's download directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Download,
  #[structopt(about = "the user's font directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "unsupported"))]
  Font,
  #[structopt(about = "the user's home directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Home,
  #[structopt(about = "the user's picture directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Picture,
  #[structopt(about = "the user's public directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Public,
  #[structopt(about = "the user's template directory")]
  #[strum(props(linux = "supported", macos = "unsupported", windows = "supported"))]
  Template,
  #[structopt(about = "the user's video directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Video,
}

impl User {
  pub fn dirs() -> Result<UserDirs> {
    UserDirs::new().ok_or(Error::InvalidHomeDirectory)
  }

  pub(crate) fn path_buf(&self) -> Result<PathBuf> {
    Self::dirs().and_then(|user| {
      match self {
        Self::Audio => user.audio_dir(),
        Self::Desktop => user.desktop_dir(),
        Self::Document => user.document_dir(),
        Self::Download => user.download_dir(),
        Self::Font => user.font_dir(),
        Self::Home => Some(user.home_dir()),
        Self::Picture => user.picture_dir(),
        Self::Public => user.public_dir(),
        Self::Template => user.template_dir(),
        Self::Video => user.video_dir(),
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
