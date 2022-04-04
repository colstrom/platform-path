use structopt::StructOpt;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Debug, StructOpt)]
pub(crate) struct FormatOptions {
  #[structopt(short = "o", long = "output-format", env = "OUTPUT_FORMAT", value_name = "format", default_value, possible_values = Format::VARIANTS)]
  pub(crate) format: Format,
}

#[derive(Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Format {
  Text,
  #[cfg(feature = "json")]
  Json,
  #[cfg(feature = "json")]
  JsonPretty,
  #[cfg(feature = "yaml")]
  Yaml,
}

impl Default for Format {
  fn default() -> Self {
    Self::Text
  }
}
