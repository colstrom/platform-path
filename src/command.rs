use structopt::StructOpt;

mod info;
mod print;
#[cfg(feature = "http")]
mod serve;

#[derive(Debug, StructOpt)]
pub enum Command {
  Print(print::PrintCommand),
  Info(info::InfoCommand),
  #[cfg(feature = "http")]
  Serve(serve::ServeCommand),
}

impl Command {
  #[cfg(feature = "async")]
  pub async fn execute(self) -> anyhow::Result<()> {
    match self {
      Self::Print(command) => command.execute()?,
      Self::Info(command) => command.execute()?,
      #[cfg(feature = "http")]
      Self::Serve(command) => command.execute().await?,
    }

    Ok(())
  }

  #[cfg(not(feature = "async"))]
  pub fn execute(self) -> anyhow::Result<()> {
    match self {
      Self::Print(command) => command.execute()?,
      Self::Info(command) => command.execute()?,
    }

    Ok(())
  }
}
