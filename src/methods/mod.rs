pub mod auth;
pub mod follow;
pub mod profile;
pub mod publication;

pub fn ping(lens_client: &crate::lens::LensClient) -> Result<String, String> {
    let query = crate::graphql::queries::Queries::new();
    let mut pong = Err(String::new());
    async_std::task::block_on(async {
        if let Ok(p) = lens_client.make_request(query.ping, None) {
            if p.status().is_success() {
                pong = Ok(String::new())
            } else {
                pong = Err(format!(
                    "Error retrieving profile with status code : {:?}",
                    p.status()
                ));
            }
        }
    });
    pong
}
