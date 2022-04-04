use crate::{Error, Result};
use camino::Utf8PathBuf;
use directories::ProjectDirs;
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
  about = "project-specific standard directories"
)]
pub enum Project {
  #[structopt(about = "the project's cache directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Cache,
  #[structopt(about = "the project's config directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Config,
  #[structopt(about = "the project's data directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Data,
  #[structopt(about = "the project's local data directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  DataLocal,
  #[structopt(about = "the project's preference directory")]
  #[strum(props(linux = "supported", macos = "supported", windows = "supported"))]
  Preference,
  #[structopt(about = "the project's path fragment")]
  PathFragment,
  #[structopt(about = "the project's runtime directory")]
  #[strum(props(linux = "supported", macos = "unsupported", windows = "unsupported"))]
  Runtime,
  #[structopt(about = "the project's state directory")]
  #[strum(props(linux = "supported", macos = "unsupported", windows = "unsupported"))]
  State,
}

#[derive(Debug, StructOpt, Clone)]
pub struct ProjectOptions {
  #[structopt(
    long = "project-qualifier",
    env = "PROJECT_QUALIFIER",
    value_name = "string",
    help = "The reverse domain name notation of the application, excluding the organization or application name itself."
  )]
  pub qualifier: Option<String>,
  #[structopt(
    long = "project-organization",
    env = "PROJECT_ORGANIZATION",
    value_name = "string",
    help = "The name of the organization that develops this application, or for which the application is developed."
  )]
  pub organization: Option<String>,
  #[structopt(
    long = "project-application",
    env = "PROJECT_APPLICATION",
    value_name = "string",
    help = "The name of the application itself."
  )]
  pub application: String,
}

impl Project {
  pub fn dirs(options: &ProjectOptions) -> Result<ProjectDirs> {
    ProjectDirs::try_from(options)
  }

  pub(crate) fn path_buf(&self, options: &ProjectOptions) -> Result<PathBuf> {
    Self::dirs(options).and_then(|project| {
      match self {
        Self::Cache => Some(project.cache_dir()),
        Self::Config => Some(project.config_dir()),
        Self::Data => Some(project.data_dir()),
        Self::DataLocal => Some(project.data_local_dir()),
        Self::Preference => Some(project.preference_dir()),
        Self::PathFragment => Some(project.project_path()),
        Self::Runtime => project.runtime_dir(),
        Self::State => project.state_dir(),
      }
      .ok_or(Error::NotDefinedByPlatformStandard)
      .map(|path| path.to_path_buf())
    })
  }

  pub fn utf8_path_buf(&self, options: &ProjectOptions) -> Result<Utf8PathBuf> {
    self
      .path_buf(options)
      .and_then(|path| Ok(Utf8PathBuf::try_from(path)?))
  }
}

impl TryFrom<&ProjectOptions> for ProjectDirs {
  type Error = crate::Error;
  fn try_from(
    ProjectOptions {
      qualifier,
      organization,
      application,
    }: &ProjectOptions,
  ) -> Result<Self> {
    let qualifier = match qualifier {
      Some(qualifier) => qualifier.as_str(),
      None => "",
    };
    let organization = match organization {
      Some(organization) => organization.as_str(),
      None => "",
    };
    ProjectDirs::from(qualifier, organization, application).ok_or(Error::InvalidHomeDirectory)
  }
}
