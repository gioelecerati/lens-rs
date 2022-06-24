pub fn challenge(
    lens_client: &crate::lens::LensClient,
    address: String,
) -> Result<crate::lens::auth::AuthChallenge, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.user.challenge;

    let mut challenge = Err(String::new());

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
                let challeng_string = p.body_string().await.unwrap();
                let challenge_data: crate::lens::auth::AuthChallenge =
                    serde_json::from_str(&challeng_string).unwrap();
                challenge = Ok(challenge_data);
            } else {
                challenge = Err(format!(
                    "Error retrieving profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    challenge
}

pub fn login(
    lens_client: &crate::lens::LensClient,
    address: String,
    signature: String,
) -> Result<crate::lens::auth::login::Login, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.user.login;

    let mut login = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![
            crate::graphql::QVar {
                name: "ADDRESS".to_string(),
                value: address,
            },
            crate::graphql::QVar {
                name: "SIGNATURE".to_string(),
                value: signature,
            },
        ],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let login_string = p.body_string().await.unwrap();
                let login_data: crate::lens::auth::login::Login =
                    serde_json::from_str(&login_string).unwrap();
                login = Ok(login_data);
            } else {
                login = Err(format!("Error login with status code : {:?}", p.status()));
            }
        }
    });
    login
}

pub fn refresh(
    lens_client: &crate::lens::LensClient,
    refresh_token: &String,
) -> Result<crate::lens::auth::refresh::RefreshData, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.user.refresh;

    let mut refresh = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![crate::graphql::QVar {
            name: "REFRESH_TOKEN".to_string(),
            value: refresh_token.clone(),
        }],
    );

    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let refresh_string = p.body_string().await.unwrap();
                let refresh_data: crate::lens::auth::refresh::RefreshData =
                    serde_json::from_str(&refresh_string).unwrap();
                refresh = Ok(refresh_data);
            } else {
                refresh = Err(format!(
                    "Error refreshing with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    refresh
}

pub fn verify(
    lens_client: &crate::lens::LensClient,
    access_token: &String,
) -> Result<crate::lens::auth::verify::VerifyData, String> {
    let queries = crate::graphql::queries::Queries::new();
    let query = queries.user.verify;

    let mut verify = Err(String::new());

    let q = crate::graphql::parse(
        query,
        vec![crate::graphql::QVar {
            name: "ACCESS_TOKEN".to_string(),
            value: access_token.clone(),
        }],
    );

    println!("QUERY: {}", q.query);
    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let verify_string = p.body_string().await.unwrap();
                let verify_data: crate::lens::auth::verify::VerifyData =
                    serde_json::from_str(&verify_string).unwrap();
                verify = Ok(verify_data);
            } else {
                verify = Err(format!(
                    "Error verifying with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    verify
}
