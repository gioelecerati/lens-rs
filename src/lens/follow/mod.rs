use serde::{Deserialize, Serialize};

pub mod followers;
pub mod following;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DFollow {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub does_follow: Vec<DoesFollow>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoesFollow {
    pub follower_address: String,
    pub profile_id: String,
    pub follows: bool,
}
