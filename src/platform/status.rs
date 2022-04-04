use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(Debug, PartialEq, Eq, Hash, Display, EnumString, EnumIter, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(
  feature = "serde1",
  derive(serde::Serialize, serde::Deserialize),
  serde(rename_all = "lowercase")
)]
pub(crate) enum Status {
  Supported,
  Unsupported,
  Unknown,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct StructuredStatus {
  pub(crate) status: Status,
}

impl From<Status> for StructuredStatus {
  fn from(status: Status) -> Self {
    Self { status }
  }
}

impl Default for Status {
  fn default() -> Self {
    Self::Unknown
  }
}

impl From<Option<&str>> for Status {
  fn from(status: Option<&str>) -> Self {
    match status {
      None => Status::Unknown,
      Some(status) => match status.parse() {
        Err(_) => Status::Unknown,
        Ok(status) => status,
      },
    }
  }
}
