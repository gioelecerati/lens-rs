use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowersData {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub followers: Followers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    pub items: Vec<Item>,
    pub page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub wallet: Wallet,
    pub total_amount_of_times_followed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub default_profile: Option<DefaultProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultProfile {
    pub id: String,
    pub name: String,
    pub bio: String,
    pub attributes: Vec<Attribute>,
    pub metadata: String,
    pub is_default: bool,
    pub handle: String,
    pub picture: Picture,
    pub cover_picture: Option<CoverPicture>,
    pub owned_by: String,
    pub dispatcher: Value,
    pub stats: Stats,
    pub follow_module: Value,
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
    pub original: Original,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub url: String,
    pub mime_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverPicture {
    pub original: Original2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original2 {
    pub url: String,
    pub mime_type: Value,
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
pub struct PageInfo {
    pub prev: String,
    pub next: String,
    pub total_count: i64,
}
