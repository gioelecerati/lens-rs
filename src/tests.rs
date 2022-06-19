// test
#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_ADDRESS: &'static str = "0x76cE9836748C5a728bFca1D6F7d8eDd6E15E4131";

    #[test]
    pub fn test() {
        let client = lens::LensClient::new(Chain::Polygon, Net::Main);
        let user_profiles = client.get_profiles_by_address(String::from(TEST_ADDRESS));

        if let Ok(profile) = user_profiles {
            if profile.data.profiles.items.len() > 0 {
                for item in profile.data.profiles.items {
                    println!("PROFILE ID: {:?}", item.id);
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
        let mut profile_id = String::new();

        if let Ok(profile) = default_profile {
            println!("DEFAULT PROFILE ID: {:?}", profile.data.default_profile.id);
            profile_id = profile.data.default_profile.id;
            assert_eq!(profile_id, "0x2dfb");
            println!(
                "DEFAULT PROFILE NAME: {:?}",
                profile.data.default_profile.name
            );
            println!(
                "DEFAULT PROFILE HANDLE: {:?}",
                profile.data.default_profile.handle
            );
            println!(
                "DEFAULT PROFILE OWNED_BY: {:?}",
                profile.data.default_profile.owned_by
            );
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

        let followers = client.get_followers(String::from(profile_id), 10);

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
    }
}
