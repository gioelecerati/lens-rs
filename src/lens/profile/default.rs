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
    pub default_profile: Option<DefaultProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultProfile {
    pub id: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub is_default: bool,
    pub attributes: Vec<Value>,
    pub metadata: Option<Value>,
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
    pub url: Option<String>,
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
