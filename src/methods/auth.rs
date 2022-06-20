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
        if let Ok(mut p) = lens_client.make_request(q,None) {
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
        if let Ok(mut p) = lens_client.make_request(q,None) {
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
