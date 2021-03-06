pub fn get_following(
    lens_client: &crate::lens::LensClient,
    address: String,
    limit: u64,
) -> Result<crate::lens::follow::following::FollowingData, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.follow.get_following;

    let mut following = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![
            crate::graphql::QVar {
                name: "ADDRESS".to_string(),
                value: address,
            },
            crate::graphql::QVar {
                name: "LIMIT".to_string(),
                value: limit.to_string(),
            },
        ],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let following_string = p.body_string().await.unwrap();
                let following_data: crate::lens::follow::following::FollowingData =
                    serde_json::from_str(&following_string).unwrap();
                following = Ok(following_data);
            } else {
                following = Err(format!(
                    "Error retrieving profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    following
}

pub fn get_followers_by_profile_id(
    lens_client: &crate::lens::LensClient,
    profile_id: String,
    limit: u64,
) -> Result<crate::lens::follow::followers::FollowersData, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.follow.get_followers;

    let mut followers = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![
            crate::graphql::QVar {
                name: "PROFILE_ID".to_string(),
                value: profile_id,
            },
            crate::graphql::QVar {
                name: "LIMIT".to_string(),
                value: limit.to_string(),
            },
        ],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let followers_string = p.body_string().await.unwrap();
                let followers_data: crate::lens::follow::followers::FollowersData =
                    serde_json::from_str(&followers_string).unwrap();
                followers = Ok(followers_data);
            } else {
                followers = Err(format!(
                    "Error retrieving profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    followers
}

pub fn does_follow(
    lens_client: &crate::lens::LensClient,
    follower_address: String,
    followed_profile_id: String,
) -> Result<crate::lens::follow::DFollow, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.follow.does_follow;

    let mut d_follow = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![
            crate::graphql::QVar {
                name: "ADDRESS".to_string(),
                value: follower_address,
            },
            crate::graphql::QVar {
                name: "PROFILE_ID".to_string(),
                value: followed_profile_id,
            },
        ],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let follow_string = p.body_string().await.unwrap();
                let follow_data: crate::lens::follow::DFollow =
                    serde_json::from_str(&follow_string).unwrap();
                d_follow = Ok(follow_data);
            } else {
                d_follow = Err(format!(
                    "Error retrieving does_follow with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    d_follow
}
