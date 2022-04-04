mod info;
mod path;
mod status;

pub(crate) use info::*;
pub use path::*;
pub(crate) use status::*;

use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

#[derive(
  Debug, PartialEq, Eq, Hash, Display, EnumString, EnumVariantNames, EnumIter, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(
  feature = "serde1",
  derive(serde::Serialize, serde::Deserialize),
  serde(rename_all = "lowercase")
)]
pub(crate) enum Platform {
  Linux,
  MacOS,
  Windows,
  Wasm,
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl Default for Platform {
  fn default() -> Self {
    Self::MacOS
  }
}

#[cfg(target_os = "windows")]
impl Default for Platform {
  fn default() -> Self {
    Self::Windows
  }
}

#[cfg(target_arch = "wasm32")]
impl Default for Platform {
  fn default() -> Self {
    Self::Wasm
  }
}

#[cfg(not(any(
  target_os = "windows",
  target_os = "macos",
  target_os = "ios",
  target_arch = "wasm32"
)))]
impl Default for Platform {
  fn default() -> Self {
    Self::Linux
  }
}
