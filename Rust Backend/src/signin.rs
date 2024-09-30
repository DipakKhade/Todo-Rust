use actix_web::{post, web, HttpResponse};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct UserCredentials {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    // exp: usize,
}

#[derive(Serialize, Deserialize)]
struct Res {
    token: String,
}

const SECRET_KEY: &[u8] = b"your-secret-key";

#[post("/signin")]
pub async fn signin_user(user: web::Json<UserCredentials>) -> HttpResponse {
    println!("Email: {}, Password: {}", user.email, user.password);

    let claims = Claims {
        sub: user.email.clone(),
        // exp: expiration,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ) {
        Ok(token) => {
            let res = Res { token };

            HttpResponse::Ok().json(res)
        }
        Err(_) => HttpResponse::Unauthorized().body("Error creating token"),
    }
}
