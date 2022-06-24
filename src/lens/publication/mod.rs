use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicationData {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub publications: Publications,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publications {
    pub items: Vec<Item>,
    pub page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub id: Option<String>,
    pub profile: Profile,
    pub stats: Stats2,
    pub metadata: Metadata,
    pub created_at: Option<String>,
    pub collect_module: CollectModule,
    pub reference_module: Value,
    pub app_id: Option<String>,
    pub hidden: bool,
    pub reaction: Value,
    pub mirrors: Option<Vec<Value>>,
    pub has_collected_by_me: bool,
    pub main_post: Option<MainPost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Option<String>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub attributes: Vec<Attribute>,
    pub is_followed_by_me: bool,
    pub is_following: bool,
    pub follow_nft_address: Option<String>,
    pub metadata: Option<String>,
    pub is_default: bool,
    pub handle: Option<String>,
    pub picture: Picture,
    pub cover_picture: Option<CoverPicture>,
    pub owned_by: Option<String>,
    pub dispatcher: Value,
    pub stats: Stats,
    pub follow_module: Option<FollowModule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub display_type: Value,
    pub trait_type: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub original: Option<Original>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub url: Option<String>,
    pub mime_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverPicture {
    pub original: Option<Original2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original2 {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowModule {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats2 {
    pub total_amount_of_mirrors: i64,
    pub total_amount_of_collects: i64,
    pub total_amount_of_comments: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub media: Vec<Medum>,
    pub attributes: Vec<Attribute2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medum {
    pub original: Option<Original3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original3 {
    pub url: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute2 {
    pub display_type: Value,
    pub trait_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectModule {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub collect_limit: Option<String>,
    pub amount: Option<Amount>,
    pub recipient: Option<String>,
    pub referral_fee: Option<i64>,
    pub end_timestamp: Option<String>,
    pub follower_only: Option<bool>,
    pub contract_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub asset: Asset,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: i64,
    pub address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPost {
    pub id: Option<String>,
    pub profile: Profile2,
    pub stats: Stats4,
    pub metadata: Metadata2,
    pub created_at: Option<String>,
    pub collect_module: CollectModule2,
    pub reference_module: Value,
    pub app_id: Option<String>,
    pub hidden: bool,
    pub reaction: Value,
    pub mirrors: Vec<Value>,
    pub has_collected_by_me: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile2 {
    pub id: Option<String>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub attributes: Vec<Attribute3>,
    pub is_followed_by_me: bool,
    pub is_following: bool,
    pub follow_nft_address: Option<String>,
    pub metadata: Option<String>,
    pub is_default: bool,
    pub handle: Option<String>,
    pub picture: Picture2,
    pub cover_picture: Option<CoverPicture2>,
    pub owned_by: Option<String>,
    pub dispatcher: Value,
    pub stats: Stats3,
    pub follow_module: Option<FollowModule2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute3 {
    pub display_type: Value,
    pub trait_type: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture2 {
    pub original: Option<Original4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original4 {
    pub url: Option<String>,
    pub mime_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverPicture2 {
    pub original: Option<Original5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original5 {
    pub url: Option<String>,
    pub mime_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats3 {
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
pub struct FollowModule2 {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats4 {
    pub total_amount_of_mirrors: i64,
    pub total_amount_of_collects: i64,
    pub total_amount_of_comments: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub media: Vec<Value>,
    pub attributes: Vec<Attribute4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute4 {
    pub display_type: Value,
    pub trait_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectModule2 {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub follower_only: Option<bool>,
    pub contract_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub prev: Option<String>,
    pub next: Option<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PublicationsType {
    OnlyComments,
    OnlyMirrors,
    OnlyPosts,
    PostAndComments,
    MirrorsAndComments,
    AllPublications,
}

impl std::fmt::Display for PublicationsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
