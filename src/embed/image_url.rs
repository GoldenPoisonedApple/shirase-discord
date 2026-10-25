//! URL for enbed image

use serde::{Serialize, Serializer};
use url::Url;

/// URL for enbed image
pub enum ImageUrl {
  /// URL
  Url(Url),
  /// Attachment
  Attachment(String),
}

impl ImageUrl {
  /// specify url
  pub fn url(url: Url) -> Self {
    Self::Url(url)
  }

  /// specify attachment filename
  ///
  /// `attachment_filename` must refer to an attachment added add_attachment method.
  pub fn attachment(attachment_filename: &str) -> Self {
    Self::Attachment(attachment_filename.to_string())
  }
}

impl Serialize for ImageUrl {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      ImageUrl::Url(url) => serializer.serialize_str(url.as_str()),
      ImageUrl::Attachment(filename) => {
        serializer.serialize_str(&format!("attachment://{}", filename))
      }
    }
  }
}
