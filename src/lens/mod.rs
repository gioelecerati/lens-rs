pub mod auth;
pub mod follow;
pub mod profile;

pub struct LensClient {
    pub queries: crate::graphql::queries::Queries,
    pub endpoint: String,
    pub chain: crate::Chain,
    pub net: crate::Net,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

/// Constructs a new LensClient
impl LensClient {
    /// Create a new LensClient
    /// # Arguments
    /// * `chain` - The chain of the Lens server
    /// * `net` - The net of the Lens server
    /// # Return
    /// * `LensClient` - The new LensClient
    pub fn new(chain: crate::Chain, net: crate::Net) -> LensClient {
        match chain {
            crate::Chain::Polygon => match net {
                crate::Net::Mumbai => LensClient {
                    queries: crate::graphql::queries::Queries::new(),
                    endpoint: String::from("https://api-mumbai.lens.dev"),
                    chain: chain,
                    net: net,
                    access_token: None,
                    refresh_token: None,
                },
                crate::Net::Main => LensClient {
                    queries: crate::graphql::queries::Queries::new(),
                    endpoint: String::from("https://api.lens.dev"),
                    chain: chain,
                    net: net,
                    access_token: None,
                    refresh_token: None,
                },
            },
        }
    }

    /// Get the profiles of a user by address
    /// # Arguments
    /// * `address` - The address of the user
    /// # Returns
    /// * `Result<profile::default::Profile, String>` - The default profile of the user
    pub fn get_default_profile_by_address(
        &self,
        address: String,
    ) -> Result<profile::default::Profile, String> {
        let profile = crate::methods::profile::get_default_profile_by_address(self, address);
        profile
    }

    /// Get the profiles of a user by address
    /// # Arguments
    /// * `address` - The address of the user
    /// # Returns
    /// * `Result<profile::AddressProfiles, String>` - The profiles of the user
    pub fn get_profiles_by_address(
        &self,
        address: String,
    ) -> Result<profile::AddressProfiles, String> {
        let profile = crate::methods::profile::get_profiles_by_address(self, address);
        profile
    }

    /// Get status of follow of a address to a profile id
    /// # Arguments
    /// * `address` - The address of the user
    /// * `followee` - The profile id of the followee
    /// # Returns
    /// * `Result<follow::DFollow, String>` - The follow status of the user
    pub fn does_follow(
        &self,
        address: String,
        followee: String,
    ) -> Result<follow::DFollow, String> {
        let follow = crate::methods::follow::does_follow(self, address, followee);
        follow
    }

    /// Get the followees of a user by address
    /// # Arguments
    /// * `address` - The address of the user
    /// * `limit` - The amount of the returned followees
    /// # Returns
    /// * `Result<follow::following::FollowingData, String>` - The followees of the user
    pub fn get_following(
        &self,
        address: String,
        limit: u64,
    ) -> Result<follow::following::FollowingData, String> {
        let follow = crate::methods::follow::get_following(self, address, limit);
        follow
    }

    /// Get the followers of a user by address
    /// # Arguments
    /// * `profile_id` - The profile id of the user
    /// * `limit` - The amount of the returned followers
    /// # Returns
    /// * `Result<follow::followers::FollowersData, String>` - The followers of the user
    pub fn get_followers(
        &self,
        profile_id: String,
        limit: u64,
    ) -> Result<follow::followers::FollowersData, String> {
        let follow = crate::methods::follow::get_followers_by_profile_id(self, profile_id, limit);
        follow
    }

    /// Retrieve the callenge to sign to login to Lens
    /// # Arguments
    /// * `address` - The address of the user
    /// # Returns
    /// * `Result<auth::challenge::Challenge, String>` - The challenge to sign to login to Lens
    pub fn challenge(&self, address: String) -> Result<auth::AuthChallenge, String> {
        let auth = crate::methods::auth::challenge(self, address);
        auth
    }

    /// Sign to login to Lens
    /// # Arguments
    /// * `address` - The address of the user
    /// * `signature` - The signature of the challenge to login to Lens
    /// # Returns
    /// * `Result<auth::login::Login, String>` - The auth token to login to Lens
    pub fn login(
        &mut self,
        address: String,
        signature: String,
    ) -> Result<auth::login::Login, String> {
        let auth = crate::methods::auth::login(self, address, signature);
        self.access_token = Some(auth.clone().unwrap().data.authenticate.access_token);
        self.refresh_token = Some(auth.clone().unwrap().data.authenticate.refresh_token);

        auth
    }

    /// Create a Lens profile
    /// # Arguments
    /// * `handle` - The handle of the user
    /// # Returns
    /// * `Result<crate::lens::profile::create::CreateProfileData, String>` - Data with transaction hash of profile creation
    pub fn create_profile(
        &self,
        handle: String,
        profile: Option<crate::lens::profile::ProfileMetadata>,
    ) -> Result<crate::lens::profile::create::CreateProfileData, String> {
        let created_profile = crate::methods::profile::create_profile(
            self,
            handle,
            crate::lens::follow::FollowModule {
                follow_type: crate::lens::follow::FollowModuleType::Free,
                currency: None,
                value: None,
                recipient: None,
            },
            profile,
        );
        created_profile
    }

    pub fn make_request(
        &self,
        q: crate::graphql::Query,
        headers: Option<Vec<serde_json::Value>>,
    ) -> Result<surf::Response, Option<String>> {
        let mut res = Err(None);
        let u = self.endpoint.clone();
        async_std::task::block_on(async {
            let mut p = surf::post(u);
            if headers.is_some() {
                let hh = headers.unwrap();
                for h in hh {
                    p = p.header(h["k"].as_str().unwrap(), h["v"].as_str().unwrap());
                }
            }
            let request = p.body_json(&q).unwrap();
            let rr = request.send().await.unwrap();
            res = Ok(rr);
        });
        res
    }
}
