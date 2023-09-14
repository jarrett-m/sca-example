use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand;

fn main() {
    // Generate an RSA key pair
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    
    // Encrypt
    let data = b"secret password";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    
    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    println!("Done!");
    println!("{}", String::from_utf8(dec_data).unwrap());
}
