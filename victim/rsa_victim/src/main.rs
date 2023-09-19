use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand;
use rsa::traits::PrivateKeyParts;
use chrono::prelude::*;

fn main() {
    // Generate an RSA key pair
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    
    // Encrypt
    let data = b"secret password";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    println!("Starting!");

    // Decrypt
    let mut dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    for _ in 0..20{
        dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
        let utc_now = Utc::now();
        println!("{}.{}.{}",utc_now.second(), utc_now.timestamp_subsec_micros(), utc_now.timestamp_subsec_nanos());
    }
    println!("Done!");
    println!("{}", String::from_utf8(dec_data).unwrap());
    println!("Secret key: {:b}", priv_key.d())
}
//time when each thingy starts