use crate::lens::follow;

pub fn create_profile(
    lens_client: &crate::lens::LensClient,
    handle: String,
    follow_module: crate::lens::follow::FollowModule,
    profile_metadata: Option<crate::lens::profile::ProfileMetadata>,
) -> Result<crate::lens::profile::create::CreateProfileData, String> {
    if lens_client.access_token.is_none() {
        return Err(String::from(
            "Cannot create a Profile without an access token.",
        ));
    }

    let queries = crate::graphql::queries::Queries::new();
    let modules = crate::graphql::queries::Module::new();

    let mut follow_module_query = crate::graphql::Query {
        query: modules.follow_module,
    };

    match follow_module.follow_type {
        follow::FollowModuleType::Fee => {
            follow_module_query = crate::graphql::parse(
                follow_module_query.query,
                vec![
                    crate::graphql::QVar {
                        name: "CURRENCY".to_string(),
                        value: follow_module.currency.unwrap(),
                    },
                    crate::graphql::QVar {
                        name: "VALUE".to_string(),
                        value: follow_module.value.unwrap(),
                    },
                    crate::graphql::QVar {
                        name: "RECIPIENT".to_string(),
                        value: follow_module.recipient.unwrap(),
                    },
                ],
            );
        }
        _ => {}
    }

    let query = queries.profile.create_profile;

    let mut variables = vec![
        crate::graphql::QVar {
            name: "HANDLE".to_string(),
            value: handle,
        },
        crate::graphql::QVar {
            name: "FOLLOW_MODULE".to_string(),
            value: follow_module_query.query,
        },
    ];
    let mut profile_picture_uri = String::from("null");
    if profile_metadata.is_some() {
        let p_metadata = profile_metadata.unwrap();
        if p_metadata.profile_picture_uri.is_some() {
            profile_picture_uri = format!("\"{}\"", p_metadata.profile_picture_uri.unwrap());
        }
    }

    variables.append(&mut vec![crate::graphql::QVar {
        name: "PROFILE_PICTURE_URI".to_string(),
        value: profile_picture_uri,
    }]);

    let q = crate::graphql::parse(query, variables);

    let mut created_profile = Err(String::new());
    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(
            q,
            Some(vec![serde_json::json!({
                "k":"x-access-token",
                "v":format!("Bearer {}",lens_client.clone().access_token.as_ref().unwrap())
            })]),
        ) {
            if p.status().is_success() {
                let created_profile_string = p.body_string().await.unwrap();
                // if HANDLE_TAKEN is present in string
                if created_profile_string.contains("HANDLE_TAKEN") {
                    created_profile = Err(String::from("Handle taken"));
                } else {
                    let created_profile_data: crate::lens::profile::create::CreateProfileData =
                        serde_json::from_str(&created_profile_string).unwrap();
                    created_profile = Ok(created_profile_data);
                }
            } else {
                created_profile = Err(format!(
                    "Error creating profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });

    created_profile
}

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
        if let Ok(mut p) = lens_client.make_request(q, None) {
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
        if let Ok(mut p) = lens_client.make_request(q, None) {
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

pub fn get_recommended_profiles(
    lens_client: &crate::lens::LensClient,
) -> Result<crate::lens::profile::recommended::RecommendedProfilesData, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.profile.recommended_profiles;

    let mut profiles = Err(String::new());
    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(crate::graphql::Query { query: query }, None) {
            if p.status().is_success() {
                let profile_string = p.body_string().await.unwrap();
                let profile_data: crate::lens::profile::recommended::RecommendedProfilesData =
                    serde_json::from_str(&profile_string).unwrap();
                profiles = Ok(profile_data);
            } else {
                profiles = Err(format!(
                    "Error retrieving recommended profiles with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    profiles
}
