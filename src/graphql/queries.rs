pub struct Queries {
    pub profile: Profile,
    pub follow: Follow,
}

pub struct Profile {
    pub get_default_profile: String,
    pub get_profiles: String,
}

pub struct Follow {
    pub does_follow: String,
    pub get_following: String,
    pub get_followers: String,
}

impl Queries {
    pub fn new() -> Queries {
        Queries {
            profile: Profile {
                get_default_profile: String::from("queries/profile/get_default_profile.graphql"),
                get_profiles: String::from("queries/profile/get_profiles.graphql"),
            },
            follow: Follow {
                does_follow: String::from("queries/follow/does_follow.graphql"),
                get_following: String::from("queries/follow/get_following.graphql"),
                get_followers: String::from("queries/follow/get_followers.graphql"),
            },
        }
    }
}
