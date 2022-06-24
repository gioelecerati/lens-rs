use lens_client;
use lens_client::{Chain, Net};

fn main() {
    // Address of registering user
    let address = String::from("0x0000000000000000000000000000000000000000");

    let mut client = lens_client::lens::LensClient::new(Chain::Polygon, Net::Mumbai);

    let challenge = client.challenge(address.clone()).unwrap();

    let challenge_text = challenge.data.challenge.text;

    // This works if you load a key store wallet using the load_wallet method or you generate a new wallet using the generate_wallet method.
    // In all the other cases, the challenge_text should be sent to the user and the user should sign the challenge_text.
    // The signature should be sent back to the server and the server should verify the signature.
    let wallet = lens_client::crypto::load_wallet(
        &"path/to/keystore/file".to_string(),
        &"filename".to_string(),
        "password".to_string(),
    )
    .unwrap();
    let challenge_signature = lens_client::crypto::sign_message(&wallet, challenge_text.as_bytes());

    // This retrieve an access_token and make it available to client.access_token
    let login_result = client.login(address.clone(), challenge_signature).unwrap();

    // Create a unique handle
    // In this example it's using the first 8 characters of the address
    let handle = address.clone()[2..10].to_lowercase();

    if let Ok(created_profile) = client.create_profile(handle.clone(), None) {
        // Return a tx_hash of the transaction
        println!(
            "Profile creating with handle {}, transaction hash: {} ",
            handle, created_profile.data.create_profile.tx_hash
        );
    } else {
        println!("Unable to create profile");
    };
}
