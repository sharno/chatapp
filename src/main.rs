use std::sync::Arc;

use axum::{extract::Query, routing::get, Json, Router};
use base64::engine::general_purpose;
use base64::prelude;
use base64::Engine;
use ed25519_dalek::Signer;
use ed25519_dalek::VerifyingKey;
use rsa::Pkcs1v15Encrypt;
use serde::Serialize;
use sqlite::Connection;

use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn generate_ed25519() -> SigningKey {
    let mut csprng = OsRng;
    return SigningKey::generate(&mut csprng);
}

use rsa::traits::PaddingScheme;
use rsa::{RsaPrivateKey, RsaPublicKey};

fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = rand::thread_rng();
    println!("generating");
    let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate a key");
    println!("generated");
    let public_key = RsaPublicKey::from(&private_key);

    (private_key, public_key)
}

// fn sign_verify(keypair: &Keypair) {
//     let message = b"hello world";
//     let signature = keypair.sign(message);

//     assert!(keypair.verify(message, &signature).is_ok());
// }
struct AppState {
    db: Connection,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let connection = sqlite::open(":memory:").unwrap();
    let state = Arc::new(AppState { db: connection });

    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";
    state.db.execute(query).unwrap();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct NewUser {
    private: RsaPrivateKey,
    public: RsaPublicKey,
}

async fn handler() -> Json<NewUser> {
    let mut rng = OsRng;
    let (priv_key, pub_key) = generate_keys();
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");

    Json::from(NewUser {
        private: priv_key,
        public: pub_key,
    })
}

// async fn handler2() -> Json<NewUser> {
//     let key = generate_ed25519();
//     // VerifyingKey::from_bytes(bytes);

//     Json::from(NewUser {
//         public: key.verifying_key(),
//         private: key,
//     })
// }
