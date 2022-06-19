use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingData {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub following: Following,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Following {
    pub items: Vec<Item>,
    pub page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub profile: Profile,
    pub total_amount_of_times_following: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub attributes: Vec<Attribute>,
    pub metadata: Option<String>,
    pub is_default: bool,
    pub handle: String,
    pub picture: Picture,
    pub cover_picture: Option<CoverPicture>,
    pub owned_by: String,
    pub dispatcher: Option<Dispatcher>,
    pub stats: Stats,
    pub follow_module: Option<FollowModule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub display_type: Option<String>,
    pub trait_type: Option<String>,
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub original: Option<Original>,
    pub medium: Option<String>,
    pub small: Option<String>,
    pub contract_address: Option<String>,
    pub token_id: Option<String>,
    pub uri: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub url: String,
    pub width: Value,
    pub height: Value,
    pub mime_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverPicture {
    pub original: Original2,
    pub small: Option<String>,
    pub medium: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original2 {
    pub url: String,
    pub width: Value,
    pub height: Value,
    pub mime_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dispatcher {
    pub address: String,
    pub can_use_relay: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_followers: i64,
    pub total_following: i64,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_mirrors: i64,
    pub total_publications: i64,
    pub total_collects: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowModule {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub prev: String,
    pub next: String,
    pub total_count: i64,
}
