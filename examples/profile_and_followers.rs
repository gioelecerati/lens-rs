use lens_client;
use lens_client::{Chain, Net};

fn main() {
    let address = "0x0000000000000000000000000000000000000000";
    let client = lens_client::lens::LensClient::new(Chain::Polygon, Net::Mumbai);

    let default_profile = client.get_default_profile_by_address(String::from(address));

    match default_profile {
        Ok(profile) => {
            let default_profile = profile.data.default_profile;
            if default_profile.is_some() {
                let profile_id = default_profile.unwrap().id;
                let followers = client.get_followers(profile_id, 10);

                // NOTE:  To get the followees of a profile, use the following method:
                // let followees = client.get_following(profile_id,10);

                match followers {
                    Ok(_followers) => {
                        // Followers are returned in a list of profiles
                    }
                    Err(e) => {}
                }
            } else {
                // User does not have a defult profile set up
                // You may retrieve a list of the address profiles using client.get_profiles_by_address(address)
                // Then use one of them
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
