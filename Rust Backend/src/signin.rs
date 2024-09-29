use actix_web::{post, web, HttpResponse, Responder};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct user_credentionals {
    email: String,
    password: String,
}

pub fn generate_token(user: user_credentionals) -> Result<String> {
    let claims = Claims {
        id: 1,
        email: user.email.clone(),
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret("JWT_SECRET"))
        .map_err(|_| Error::JWTTokenCreationError)
}

#[post("/signin")]
pub async fn signin_user(user: web::Json<user_credentionals>) -> HttpResponse {
    println!("{:?} {}", user.email, user.password);
    HttpResponse::Ok().json(user)
}
