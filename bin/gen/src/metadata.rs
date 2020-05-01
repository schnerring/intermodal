use crate::common::*;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub(crate) struct Metadata {
  #[serde(rename = "type")]
  pub(crate) kind: Kind,
  #[serde(default)]
  pub(crate) pr: Option<Url>,
  #[serde(default)]
  pub(crate) fixes: Vec<Url>,
  #[serde(default, rename = "co-authored-by")]
  pub(crate) co_authored_by: Option<String>,
}

impl Metadata {
  #[throws]
  pub(crate) fn from_commit(commit: &Commit) -> Metadata {
    const BLANK: &str = "\n\n";

    let message = String::from_utf8_lossy(commit.message_bytes());

    let blank = message
      .rfind(BLANK)
      .ok_or_else(|| Error::CommitMetadataMissing {
        hash: commit.id(),
        message: message.to_string(),
      })?;

    let yaml = &message[blank + BLANK.len()..];

    let metadata = serde_yaml::from_str(yaml).context(error::CommitMetadataDeserialize {
      hash: commit.id(),
      message,
    })?;

    metadata
  }
}

impl Default for Metadata {
  fn default() -> Metadata {
    Metadata {
      kind: Kind::Changed,
      pr: None,
      fixes: Vec::new(),
      co_authored_by: None,
    }
  }
}

impl Display for Metadata {
  #[throws(fmt::Error)]
  fn fmt(&self, f: &mut Formatter) {
    writeln!(f)?;
    writeln!(
      f,
      "{}",
      serde_yaml::to_string(&self)
        .unwrap()
        .split("---")
        .last()
        .unwrap()
        .trim()
    )?;
  }
}
