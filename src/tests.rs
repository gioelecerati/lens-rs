// test
#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_ADDRESS: &'static str = "0x76cE9836748C5a728bFca1D6F7d8eDd6E15E4131";

    #[test]
    pub fn test_existing_user() {
        let client = lens::LensClient::new(Chain::Polygon, Net::Main);
        let user_profiles = client.get_profiles_by_address(String::from(TEST_ADDRESS));

        let mut profile_id = String::new();
        if let Ok(profile) = user_profiles {
            if profile.data.profiles.items.len() > 0 {
                for item in profile.data.profiles.items {
                    println!("PROFILE ID: {:?}", item.id);
                    profile_id = item.id;
                    println!("PROFILE NAME: {:?}", item.name);

                    assert_eq!(item.handle, "gioele.lens");

                    println!("PROFILE HANDLE: {:?}", item.handle);
                    println!("PROFILE OWNED_BY: {:?}", item.owned_by);
                }
            } else {
                println!("No profiles found");
            }
        }

        let default_profile = client.get_default_profile_by_address(String::from(TEST_ADDRESS));

        if let Ok(profile) = default_profile {
            if profile.data.default_profile.is_some() {
                let p = profile.data.default_profile.unwrap();
                println!("DEFAULT PROFILE ID: {:?}", p.id);
                profile_id = p.id;
                assert_eq!(profile_id, "0x2dfb");
                println!("DEFAULT PROFILE NAME: {:?}", p.name);
                println!("DEFAULT PROFILE HANDLE: {:?}", p.handle);
                println!("DEFAULT PROFILE OWNED_BY: {:?}", p.owned_by);
            } else {
                println!("No default profile found");
            }
        }

        let following = client.get_following(String::from(TEST_ADDRESS), 10);

        let mut following_list: Vec<String> = Vec::new();
        if let Ok(following) = following {
            if following.data.following.items.len() > 0 {
                for item in following.data.following.items {
                    following_list.push(item.profile.handle);
                }
            } else {
                println!("No following found");
            }
        }

        println!("FOLLOWING LIST {:?}", following_list);

        let followers = client.get_followers(String::from(profile_id.clone()), 10);

        let mut followers_list: Vec<String> = Vec::new();
        if let Ok(followers) = followers {
            if followers.data.followers.items.len() > 0 {
                for item in followers.data.followers.items {
                    if item.wallet.default_profile.is_some() {
                        let follower = item.wallet.default_profile.unwrap().handle;

                        followers_list.push(follower);
                    } else {
                        followers_list.push(item.wallet.address);
                    }
                }
            } else {
                println!("No followers found");
            }
        }

        println!("FOLLOWERS LIST {:?}", followers_list);

        if let Ok(challenge) = client.challenge(String::from(TEST_ADDRESS)) {
            println!("CHALLENGE: {:?}", challenge);
        } else {
            println!("Unable to generate challenge");
        }

        let publications = client.get_publications(
            profile_id.clone(),
            10,
            lens::publication::PublicationsType::OnlyComments,
        );

        println!("PUBLICATIONS: {:?}", publications);
    }

    #[test]
    fn test_new_user() {
        let folder_path = "/tmp";
        let file_name = "lens_rs_wallet";

        let mut _w: Option<crate::crypto::Wallet> = None;

        if std::path::Path::new(&format!("{}/{}", folder_path, file_name)).exists() {
            _w = Some(
                crate::crypto::load_wallet(
                    &folder_path.to_string(),
                    &file_name.to_string(),
                    String::from("password"),
                )
                .unwrap(),
            );
        } else {
            _w = Some(
                crypto::generate_wallet(
                    String::from(folder_path),
                    String::from("password"),
                    Some(&file_name),
                    true,
                )
                .unwrap(),
            );
        }

        let wallet = _w.unwrap();

        println!("{:?}", wallet.address);

        let user_address = wallet.address.clone();

        println!(
            "Generated wallet: {} in path {}, with name {}",
            user_address,
            wallet.path.clone(),
            wallet.name
        );

        let signature = crypto::sign_message(&wallet, String::from("test").as_bytes());

        println!("Test signature: {}", signature);

        let loaded_wallet =
            crypto::load_wallet(&wallet.path, &wallet.name, String::from("password")).unwrap();

        let loaded_signature =
            crypto::sign_message(&loaded_wallet, String::from("test").as_bytes());
        println!("Test loaded signature: {}", loaded_signature);
        assert_eq!(signature, loaded_signature);

        let mut client = lens::LensClient::new(Chain::Polygon, Net::Mumbai);

        let challenge = client.challenge(user_address.clone()).unwrap();
        let challenge_text = challenge.data.challenge.text;

        println!("{}", challenge_text);

        println!("Signing challenge..{}", challenge_text);
        let challenge_signature = crypto::sign_message(&wallet, challenge_text.as_bytes());
        println!("Challenge signature: {}", challenge_signature);

        let login_result = client
            .login(user_address.clone(), challenge_signature)
            .unwrap();

        println!("Login result: {:?}", login_result);
        let handle = user_address.clone()[2..10].to_lowercase();
        if let Ok(created_profile) = client.create_profile(handle.clone(), None) {
            println!(
                "Profile creating with handle {}, transaction hash: {} ",
                handle, created_profile.data.create_profile.tx_hash
            );
        } else {
            println!("Unable to create profile");
        };

        //let verify_token = client.verify().unwrap();
        //println!("Verify token: {:?}", verify_token);
        let refresh_token = client.refresh().unwrap();
        println!("Refresh token: {:?}", refresh_token);

        let get_profiles = client.get_profiles_by_address(user_address.clone());

        if let Ok(profiles) = get_profiles {
            if profiles.data.profiles.items.len() > 0 {
                for item in profiles.data.profiles.items {
                    println!("PROFILE ID: {:?}", item.id);
                    println!("PROFILE NAME: {:?}", item.name);
                    println!("PROFILE HANDLE: {:?}", item.handle);
                    println!("PROFILE OWNED_BY: {:?}", item.owned_by);
                }
            } else {
                println!("No profiles found");
            }
        }
    }

    #[test]
    fn test_recommended_profiles() {
        let lens_client = lens::LensClient::new(Chain::Polygon, Net::Mumbai);
        let recommended_profiles = lens_client.get_recommended_profiles();
        println!("Recommended profiles: {:?}", recommended_profiles);
    }
}
