use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub default_profile: DefaultProfile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultProfile {
    pub id: String,
    pub name: String,
    pub bio: String,
    pub is_default: bool,
    pub attributes: Vec<Value>,
    pub metadata: Value,
    pub handle: String,
    pub picture: Picture,
    pub cover_picture: Value,
    pub owned_by: String,
    pub dispatcher: Value,
    pub stats: Stats,
    pub follow_module: Value,
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
pub struct Stats {
    pub total_followers: i64,
    pub total_following: i64,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_mirrors: i64,
    pub total_publications: i64,
    pub total_collects: i64,
}
