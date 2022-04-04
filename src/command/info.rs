use crate::output::FormatOptions;
use crate::platform::PlatformPathKind as Kind;
use crate::platform::{Info, InfoFilter, Platform, Status};
use structopt::StructOpt;
use strum::{IntoEnumIterator, VariantNames};

#[derive(Debug, StructOpt)]
pub struct InfoCommand {
  #[structopt(flatten)]
  filter: FilterOptions,
  #[structopt(flatten)]
  format: FormatOptions,
}

#[derive(Debug, Default, StructOpt)]
struct FilterOptions {
  #[structopt(short = "p", long = "platform", possible_values = Platform::VARIANTS)]
  platform: Option<Platform>,
  #[structopt(short = "k", long = "kind", possible_values = Kind::VARIANTS)]
  kind: Option<Kind>,
  #[structopt(short = "s", long = "status", possible_values = Status::VARIANTS)]
  status: Option<Status>,
}

impl InfoCommand {
  pub fn execute(self) -> anyhow::Result<()> {
    let Self {
      filter,
      format: FormatOptions { format, .. },
    } = self;

    let filter = filter.into();
    let info = Info::filtered(&filter).render(&format)?;

    println!("{info}");

    Ok(())
  }
}

impl From<FilterOptions> for InfoFilter {
  fn from(options: FilterOptions) -> Self {
    let platforms = match options.platform {
      Some(platform) => vec![platform],
      None => Platform::iter().collect(),
    };

    let paths = match options.kind {
      Some(path) => vec![path],
      None => Kind::iter().collect(),
    };

    let statuses = match options.status {
      Some(status) => vec![status],
      None => Status::iter().collect(),
    };

    InfoFilter {
      platforms,
      paths,
      statuses,
    }
  }
}
