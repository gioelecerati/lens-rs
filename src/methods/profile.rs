
pub fn get_default_profile_by_address(
    lens_client: &crate::lens::LensClient,
    address: String,
) -> Result<crate::lens::profile::default::Profile, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.profile.get_default_profile;

    let mut profile = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![crate::graphql::QVar {
            name: "ADDRESS".to_string(),
            value: address,
        }],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q) {
            if p.status().is_success() {
                let profile_string = p.body_string().await.unwrap();
                let profile_data: crate::lens::profile::default::Profile =
                    serde_json::from_str(&profile_string).unwrap();
                profile = Ok(profile_data);
            } else {
                profile = Err(format!(
                    "Error retrieving profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    profile
}

pub fn get_profiles_by_address(
    lens_client: &crate::lens::LensClient,
    address: String,
) -> Result<crate::lens::profile::AddressProfiles, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.profile.get_profiles;

    let mut profile = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![crate::graphql::QVar {
            name: "ADDRESS".to_string(),
            value: address,
        }],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q) {
            if p.status().is_success() {
                let profile_string = p.body_string().await.unwrap();
                let profile_data: crate::lens::profile::AddressProfiles =
                    serde_json::from_str(&profile_string).unwrap();
                profile = Ok(profile_data);
            } else {
                profile = Err(format!(
                    "Error retrieving profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    profile
}
