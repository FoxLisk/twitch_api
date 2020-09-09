//! Endpoints regarding tags
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, tags::GetAllStreamTagsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetAllStreamTagsRequest::builder()
//!     .build();
//!
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

#[doc(inline)]
pub use get_all_stream_tags::{GetAllStreamTagsRequest, Tag};

use crate::helix;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use typed_builder::TypedBuilder;

/// Language code, formatted as 2 letter language by ISO 639-1, a dash (`-`) and 2 letter region by ISO 3166-1
///
/// i.e
/// `en-us`
/// `bg-bg`
/// etc etc
pub type Language = String;

/// Tag is auto-generated or not.
#[derive(Clone, Debug, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(from = "bool")]
#[serde(into = "bool")]
pub enum AutoGenerated {
    /// Was auto-generated
    True,
    /// Was not auto-generated
    False,
}

impl From<bool> for AutoGenerated {
    fn from(v: bool) -> Self {
        match v {
            true => AutoGenerated::True,
            false => AutoGenerated::False,
        }
    }
}

impl From<AutoGenerated> for bool {
    fn from(v: AutoGenerated) -> Self {
        match v {
            AutoGenerated::True => true,
            AutoGenerated::False => false,
        }
    }
}
/// A stream tag as defined by Twitch.
#[derive(PartialEq, Deserialize, Debug, Clone)]
pub struct TwitchTag {
    /// ID of the tag.
    #[serde(alias = "tag_id")]
    pub id: String,
    /// true if the tag is auto-generated; otherwise, false . An auto-generated tag is one automatically applied by Twitch (e.g., a language tag based on the broadcaster’s settings); these cannot be added or removed by the user.
    pub is_auto: AutoGenerated,
    /// All localized names of the tag.
    pub localization_names: BTreeMap<Language, String>,
    /// All localized descriptions of the tag.
    pub localization_descriptions: BTreeMap<Language, String>,
}

/// Gets the list of all stream tags defined by Twitch, optionally filtered by tag ID(s).
/// [`get-all-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags)
pub mod get_all_stream_tags {
    use super::*;

    /// Query Parameters for [Get All Stream Tags](super::get_all_stream_tags)
    ///
    /// [`get-all-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetAllStreamTagsRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
        /// ID of a tag. Multiple IDs can be specified. If provided, only the specified tag(s) is(are) returned. Maximum of 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub tag_id: Vec<String>,
    }

    /// Return Values for [Get All Stream Tags](super::get_all_stream_tags)
    ///
    /// [`get-all-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags)
    pub type Tag = helix::tags::TwitchTag;

    impl helix::Request for GetAllStreamTagsRequest {
        type Response = Tag;

        const PATH: &'static str = "tags/streams";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetAllStreamTagsRequest {}

    impl helix::Paginated for GetAllStreamTagsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor); }
    }
}