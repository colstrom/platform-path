use crate::output::Format;
use crate::platform::path::{Base, Project, User};
use crate::platform::PlatformPathKind as Kind;
use crate::platform::{Platform, PlatformPath, Status, StructuredStatus};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct InfoFilter {
  pub(crate) platforms: Vec<Platform>,
  pub(crate) paths: Vec<Kind>,
  pub(crate) statuses: Vec<Status>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct Info(HashMap<Platform, HashMap<Kind, HashMap<PlatformPath, StructuredStatus>>>);

impl Info {
  pub(crate) fn filtered(filter: &InfoFilter) -> Self {
    let mut platforms = HashMap::new();

    for platform in Platform::iter().filter(|platform| filter.platforms.contains(platform)) {
      let mut kinds = HashMap::new();

      for kind in Kind::iter().filter(|kind| filter.paths.contains(kind)) {
        let mut paths = HashMap::new();

        let variant: Vec<PlatformPath> = match &kind {
          Kind::Base => Base::iter().map(PlatformPath::from).collect(),
          Kind::User => User::iter().map(PlatformPath::from).collect(),
          Kind::Project => Project::iter().map(PlatformPath::from).collect(),
        };

        for path in variant {
          let status = path.status(&platform);
          if filter.statuses.contains(&status) {
            let status = StructuredStatus { status };
            paths.insert(path, status);
          }
        }

        kinds.insert(kind, paths);
      }

      platforms.insert(platform, kinds);
    }

    Self(platforms)
  }

  pub(crate) fn render(&self, format: &Format) -> crate::Result<String> {
    let rendered = match format {
      #[cfg(feature = "json")]
      Format::Json => serde_json::to_string(self)?,
      #[cfg(feature = "json")]
      Format::JsonPretty => serde_json::to_string_pretty(self)?,
      #[cfg(feature = "yaml")]
      Format::Yaml => serde_yaml::to_string(self)?,
      Format::Text => {
        let mut records = Vec::<String>::new();
        for (platform, kinds) in self.0.iter() {
          for (kind, paths) in kinds {
            for (path, StructuredStatus { status }) in paths {
              let path = path.to_string();
              records.push(format!("{platform} {kind} {path} {status}"));
            }
          }
        }
        records.sort();
        records.join("\n")
      }
    };

    Ok(rendered)
  }
}
