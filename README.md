# lens-rs

Rust client crate for https://lens.xyz  
Just a draft. See on the bottom to see which APIs it currently supports.
The GraphQL query templates are currently hardcoded in to the code into grapql::queries
## Usage

```Rust
use lens_rs;
use lens_rs::{Chain, Net};

fn main(){
    let address = "0x0000000000000000000000000000000000000000";
    let client = lens_rs::lens::LensClient::new(Chain::Polygon, Net::Main);

    let default_profile = client.get_default_profile_by_address(String::from(address));

    match default_profile {
        Ok(profile) => {
            println!("{:?}", profile);
            let profile_id = profile.data.default_profile.id;
            let followers = client.get_followers(profile_id,10);

            match followers {
                Ok(followers) => {
                    println!("{:?}", followers);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
```

### Functionalities

#### Profile
- [X] Profile : get default profile
- [X] Profile : get profiles
- [X] Profile : create profile
#### Follow
- [X] Follow  : does follow
- [X] Follow  : get followers
- [X] Follow  : get following 
#### Auth
- [X] Auth    : get challenge
- [X] Auth    : login

This crate integrates some functions to handle local KeyStores and use them to interact with Lens, mostly useful for development purposes.
