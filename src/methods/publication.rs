use convert_case::{Case, Casing};

pub fn get_publications(
    lens_client: &crate::lens::LensClient,
    profile_id: String,
    limit: i64,
    publication_types: crate::lens::publication::PublicationsType,
) -> Result<crate::lens::publication::PublicationData, String> {
    let queries = crate::graphql::queries::Queries::new();
    let modules = crate::graphql::queries::Module::new();

    let publication_module_req = publication_types.to_string().to_case(Case::Snake);

    let publication_module: String =
        crate::utils::get_field_by_name(&modules, &publication_module_req);

    let query = queries.publication.get_publications;

    let variables = vec![
        crate::graphql::QVar {
            name: "PROFILE_ID".to_string(),
            value: profile_id,
        },
        crate::graphql::QVar {
            name: "PUBLICATION_TYPES".to_string(),
            value: publication_module,
        },
        crate::graphql::QVar {
            name: "LIMIT".to_string(),
            value: limit.to_string(),
        },
    ];

    let q = crate::graphql::parse(query, variables);

    let mut publications = Err(String::new());
    async_std::task::block_on(async {
        if let Ok(mut p) = lens_client.make_request(q, None) {
            if p.status().is_success() {
                let publications_string = p.body_string().await.unwrap();
                // if HANDLE_TAKEN is present in string
                if publications_string.contains("HANDLE_TAKEN") {
                    publications = Err(String::from("Handle taken"));
                } else {
                    let publications_data: crate::lens::publication::PublicationData =
                        serde_json::from_str(&publications_string).unwrap();
                    publications = Ok(publications_data);
                }
            } else {
                publications = Err(format!(
                    "Error getting publications with status code : {:?}",
                    p.status()
                ));
            }
        }
    });

    publications
}
