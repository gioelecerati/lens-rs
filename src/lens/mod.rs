pub mod follow;
pub mod profile;
pub mod auth;

pub struct LensClient {
    pub queries: crate::graphql::queries::Queries,
    pub endpoint: String,
    pub chain: crate::Chain,
    pub net: crate::Net,
}

impl LensClient {
    pub fn new(chain: crate::Chain, net: crate::Net) -> LensClient {
        match chain {
            crate::Chain::Polygon => match net {
                crate::Net::Mumbai => LensClient {
                    queries: crate::graphql::queries::Queries::new(),
                    endpoint: String::from("https://api-mumbai.lens.dev"),
                    chain: chain,
                    net: net,
                },
                crate::Net::Main => LensClient {
                    queries: crate::graphql::queries::Queries::new(),
                    endpoint: String::from("https://api.lens.dev"),
                    chain: chain,
                    net: net,
                },
            },
        }
    }

    pub fn get_default_profile_by_address(
        &self,
        address: String,
    ) -> Result<profile::default::Profile, String> {
        let profile = crate::methods::profile::get_default_profile_by_address(self, address);
        profile
    }

    pub fn get_profiles_by_address(
        &self,
        address: String,
    ) -> Result<profile::AddressProfiles, String> {
        let profile = crate::methods::profile::get_profiles_by_address(self, address);
        profile
    }

    pub fn does_follow(
        &self,
        address: String,
        followee: String,
    ) -> Result<follow::DFollow, String> {
        let follow = crate::methods::follow::does_follow(self, address, followee);
        follow
    }

    pub fn get_following(
        &self,
        address: String,
        limit: u64,
    ) -> Result<follow::following::FollowingData, String> {
        let follow = crate::methods::follow::get_following(self, address, limit);
        follow
    }

    pub fn get_followers(
        &self,
        profile_id: String,
        limit: u64,
    ) -> Result<follow::followers::FollowersData, String> {
        let follow = crate::methods::follow::get_followers_by_profile_id(self, profile_id, limit);
        follow
    }

    pub fn challenge(
        &self,
        address: String,
    ) -> Result<auth::AuthChallenge, String> {
        let auth = crate::methods::auth::challenge(self, address);
        auth
    }
    pub fn make_request(&self, q: crate::graphql::Query) -> Result<surf::Response, Option<String>> {
        let mut res = Err(None);
        let u = self.endpoint.clone();
        async_std::task::block_on(async {
            let response = surf::post(u).body_json(&q).unwrap().await.unwrap();
            res = Ok(response);
        });
        res
    }

}


