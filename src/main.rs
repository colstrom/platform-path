use platform_path::Command;
use structopt::StructOpt;

#[cfg(feature = "async")]
#[async_std::main]
async fn main() -> anyhow::Result<()> {
  Command::from_args().execute().await?;

  Ok(())
}

#[cfg(not(feature = "async"))]
fn main() -> anyhow::Result<()> {
  Command::from_args().execute()?;

  Ok(())
}
