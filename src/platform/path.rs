mod base;
mod project;
mod user;

use super::{Platform, Status};
pub use base::*;
pub use project::*;
pub use user::*;

use strum::{Display, EnumIter, EnumProperty, EnumString, EnumVariantNames};

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde1",
  derive(serde::Serialize, serde::Deserialize),
  serde(untagged)
)]
pub(crate) enum PlatformPath {
  Base(Base),
  User(User),
  Project(Project),
}

impl PlatformPath {
  pub(crate) fn status(&self, platform: &Platform) -> Status {
    let prop = platform.into();
    match self {
      Self::Base(variant) => variant.get_str(prop),
      Self::User(variant) => variant.get_str(prop),
      Self::Project(variant) => variant.get_str(prop),
    }
    .into()
  }
}

impl ToString for PlatformPath {
  fn to_string(&self) -> String {
    match self {
      Self::Base(variant) => variant.to_string(),
      Self::User(variant) => variant.to_string(),
      Self::Project(variant) => variant.to_string(),
    }
  }
}

impl From<Base> for PlatformPath {
  fn from(path: Base) -> Self {
    Self::Base(path)
  }
}

impl From<User> for PlatformPath {
  fn from(path: User) -> Self {
    Self::User(path)
  }
}

impl From<Project> for PlatformPath {
  fn from(path: Project) -> Self {
    Self::Project(path)
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Display, EnumString, EnumVariantNames, EnumIter)]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(
  feature = "serde1",
  derive(serde::Serialize, serde::Deserialize),
  serde(rename_all = "lowercase")
)]
pub(crate) enum PlatformPathKind {
  Base,
  User,
  Project,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct StructuredPathString {
  pub(crate) path: String,
}

impl From<String> for StructuredPathString {
  fn from(path: String) -> Self {
    Self { path }
  }
}
