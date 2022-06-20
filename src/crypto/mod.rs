use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use std::io::Read;
use std::path::Path;

use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub struct Wallet {
    pub keystore: eth_keystore::EthKeystore,
    pub private_key: Option<Vec<u8>>,
    pub public_key: Option<PublicKey>,
    pub name: String,
    pub signing_key: SigningKey,
    pub address: String,
    pub path: String,
}

/// Generate a new wallet with a random private key.
/// The wallet will be saved to the given path.
/// The wallet will be encrypted with the given password.
/// NOTE: only for development purposes or applications running on your user device.
/// DO NOT use this in your backend of a web application, as the wallet is stored on your backend.
///
/// Probably unsafe to use in production. Always prefer to just receive signatures from the user client.
pub fn generate_wallet(
    path: String,
    password: String,
    name: Option<&str>,
    with_keys: bool,
) -> Result<Wallet, eth_keystore::KeystoreError> {
    let mut rng = rand::thread_rng();
    let dir = Path::new(&path);
    let (_private_key, name) = eth_keystore::new(dir, &mut rng, password.clone(), name)?;

    let mut file = std::fs::File::open(format!("{}/{}", path, name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let wallet = keystore_to_wallet(contents, &path, &name, &password, with_keys)?;

    return Ok(wallet);
}

pub fn load_wallet(
    path: &String,
    name: &String,
    password: String,
) -> Result<Wallet, eth_keystore::KeystoreError> {
    let mut file = std::fs::File::open(format!("{}/{}", path, name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let wallet = keystore_to_wallet(contents, path, name, &password, true)?;

    return Ok(wallet);
}

pub fn keystore_to_wallet(
    keystore_string: String,
    path: &String,
    name: &String,
    password: &String,
    with_keys: bool,
) -> Result<Wallet, eth_keystore::KeystoreError> {
    let keystore: eth_keystore::EthKeystore = serde_json::from_str(&keystore_string)?;

    let private_key = eth_keystore::decrypt_key(format!("{}/{}", path.clone(), name), password)?;

    let mut k = None;
    let mut pk = None;
    let signing_key = k256::ecdsa::SigningKey::from_bytes(&private_key).unwrap();

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key).expect("32 bytes, within curve order");
    let secp256k1_public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let pub_key_bytes = &secp256k1_public_key.serialize_uncompressed()[1..];

    let mut hasher: Keccak256 = Keccak256::new();
    hasher.update(pub_key_bytes);
    let hash = hasher.finalize();

    let hash_hex = hex::encode(hash).to_string();

    let address = eth_checksum::checksum(&format!("0x{}", &hash_hex[24..]));

    if with_keys {
        k = Some(private_key);
        pk = Some(secp256k1_public_key);
    }

    let wallet = Wallet {
        keystore,
        private_key: k,
        public_key: pk,
        name: name.to_string(),
        signing_key: signing_key,
        address: address,
        path: path.clone(),
    };

    Ok(wallet)
}

use ethers_signers::LocalWallet;
use ethers_signers::Signer as EthSigner;

/// Main sign function
/// Takes a message and returns a signature
pub fn sign_message(wallet: &Wallet, message: &[u8]) -> String {
    let mut signature = String::new();
    async_std::task::block_on(async {
        let wallet = hex::encode(&wallet.private_key.as_ref().unwrap())
            .parse::<LocalWallet>()
            .unwrap();

        signature = wallet.sign_message(message).await.unwrap().to_string();
    });

    let hex_signature = signature;

    return format!("{}", hex_signature);
}
