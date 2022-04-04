use crate::platform::PlatformPathKind as Kind;
use crate::{Base, Project, ProjectOptions, User};
use camino::Utf8PathBuf;
use structopt::StructOpt;

#[cfg(feature = "http")]
use tide::{Request, Server};

#[derive(Debug, StructOpt)]
#[structopt(about = "expose path info over HTTP")]
pub struct ServeCommand {
  #[structopt(flatten)]
  state: ProjectOptions,
  #[structopt(flatten)]
  router: RouterOptions,
  #[structopt(flatten)]
  listener: ListenerOptions,
}

#[derive(Debug, StructOpt)]
struct RouterOptions {
  #[structopt(
    long = "route-prefix",
    env = "ROUTE_PREFIX",
    value_name = "path",
    default_value = "/platform-path",
    help = "serve requests under this prefix"
  )]
  prefix: String,
}

#[derive(Debug, StructOpt)]
struct ListenerOptions {
  #[structopt(
    long = "listen-address",
    env = "LISTEN_ADDRESS",
    value_name = "address",
    default_value = "127.0.0.1:8411"
  )]
  address: String,
  #[cfg(feature = "https")]
  #[structopt(flatten)]
  tls: TlsOptions,
}

#[cfg(feature = "https")]
#[derive(Debug, StructOpt)]
struct TlsOptions {
  #[structopt(
    long = "with-tls",
    takes_value = false,
    requires = "tls-certificate-path",
    requires = "tls-key-path",
    help = "enable TLS for this endpoint"
  )]
  enabled: bool,

  #[structopt(
    long = "tls-certificate-path",
    env = "TLS_CERTIFICATE_PATH",
    value_name = "Path",
    requires = "tls-key-path",
    help = "path to a TLS certificate file"
  )]
  certificate_path: Option<Utf8PathBuf>,

  #[structopt(
    long = "tls-key-path",
    env = "TLS_KEY_PATH",
    value_name = "Path",
    requires = "tls-certificate-path"
  )]
  key_path: Option<Utf8PathBuf>,
}

impl ServeCommand {
  pub async fn execute(self) -> anyhow::Result<()> {
    let Self {
      state,
      router,
      listener,
    } = self;
    let mut server = tide::with_state(state);
    router.route(&mut server);
    listener.listen(server).await?;
    Ok(())
  }
}

impl RouterOptions {
  fn route(self, server: &mut Server<ProjectOptions>) {
    let prefix = if self.prefix.starts_with('/') {
      self.prefix
    } else {
      format!("/{}", self.prefix)
    };

    #[cfg(feature = "http")]
    server
      .at(&format!("{prefix}/v0/:kind/:path/text"))
      .get(text);
    #[cfg(all(feature = "http", feature = "json"))]
    server
      .at(&format!("{prefix}/v0/:kind/:path/json"))
      .get(json);
    #[cfg(all(feature = "http", feature = "yaml"))]
    server
      .at(&format!("{prefix}/v0/:kind/:path/yaml"))
      .get(yaml);
  }
}

impl ListenerOptions {
  #[cfg(feature = "https")]
  async fn listen(self, server: Server<ProjectOptions>) -> anyhow::Result<()> {
    use tide_rustls::TlsListener;
    if self.tls.enabled {
      match (self.tls.certificate_path, self.tls.key_path) {
            (Some(tls_certificate_path), Some(tls_key_path)) => {
              server.listen(TlsListener::build()
                .addrs(&self.address)
                .cert(&tls_certificate_path)
                .key(&tls_key_path)).await?
              },
              _ => panic!("Missing either certificate or key. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻"),
            }
    } else {
      server.listen(&self.address).await?
    }

    Ok(())
  }

  #[cfg(not(feature = "https"))]
  async fn listen(self, server: Server<ProjectOptions>) -> anyhow::Result<()> {
    server.listen(self.address).await?;

    Ok(())
  }
}

#[cfg(feature = "http")]
fn handle(request: Request<ProjectOptions>) -> tide::Result<Utf8PathBuf> {
  let path = request.param("path")?;
  let path = match request.param("kind")?.parse::<Kind>()? {
    Kind::Base => path.parse::<Base>()?.utf8_path_buf(),
    Kind::User => path.parse::<User>()?.utf8_path_buf(),
    Kind::Project => path.parse::<Project>()?.utf8_path_buf(request.state()),
  }?;
  Ok(path)
}

#[cfg(feature = "http")]
async fn text(request: Request<ProjectOptions>) -> tide::Result {
  let path = handle(request)?;
  Ok(path.into_string().into())
}

#[cfg(all(feature = "http", feature = "json"))]
async fn json(request: Request<ProjectOptions>) -> tide::Result {
  use crate::platform::StructuredPathString;

  let path: String = handle(request)?.into();
  Ok(serde_json::to_string(&StructuredPathString::from(path))?.into())
}

#[cfg(all(feature = "http", feature = "yaml"))]
async fn yaml(request: Request<ProjectOptions>) -> tide::Result {
  use crate::platform::StructuredPathString;

  let path: String = handle(request)?.into();
  Ok(serde_yaml::to_string(&StructuredPathString::from(path))?.into())
}
