use biscuit_auth::KeyPair;

fn main() {
    let root = KeyPair::new();
    let private_key = root.private();
    println!("{}", base64::encode(private_key.to_bytes()));
}
