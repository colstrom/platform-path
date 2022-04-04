use crate::output::{Format, FormatOptions};
use crate::platform::{Base, Project, ProjectOptions, StructuredPathString, User};
use crate::Result;
use camino::Utf8PathBuf;
use std::path::PathBuf;
use structopt::StructOpt;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Debug, StructOpt)]
#[structopt(about = "print path info to STDOUT")]
pub struct PrintCommand {
  #[structopt(long, help = "alternate path to use in case of errors")]
  default: Option<PathBuf>,
  #[structopt(long, value_name = "mode", default_value, possible_values = Unicode::VARIANTS)]
  unicode: Unicode,
  #[structopt(flatten)]
  format: FormatOptions,
  #[structopt(subcommand)]
  path: PlatformPath,
}

impl PrintCommand {
  pub fn execute(self) -> Result<()> {
    let Self {
      default,
      unicode,
      format: FormatOptions { format },
      path,
    } = self;

    let path = match path {
      PlatformPath::User(path) => path.path_buf(),
      PlatformPath::Base(path) => path.path_buf(),
      PlatformPath::Project { path, options } => path.path_buf(&options),
    }
    .or_else(|err| default.ok_or(err))?;

    let path = match unicode {
      Unicode::Required => Utf8PathBuf::try_from(path)?.to_string(),
      Unicode::Enforced => path.to_string_lossy().to_string(),
    };

    let path = StructuredPathString::from(path);
    let output = match format {
      Format::Text => path.path,
      #[cfg(feature = "json")]
      Format::Json => serde_json::to_string(&path)?,
      #[cfg(feature = "json")]
      Format::JsonPretty => serde_json::to_string_pretty(&path)?,
      #[cfg(feature = "yaml")]
      Format::Yaml => serde_yaml::to_string(&path)?,
    };

    println!("{output}");
    Ok(())
  }
}

#[derive(Debug, StructOpt)]
enum PlatformPath {
  User(User),
  Base(Base),
  Project {
    #[structopt(subcommand)]
    path: Project,
    #[structopt(flatten)]
    options: ProjectOptions,
  },
}

#[derive(Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum Unicode {
  Required,
  Enforced,
}

impl Default for Unicode {
  fn default() -> Self {
    Self::Required
  }
}
