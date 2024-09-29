use actix_web::{post, HttpResponse, Responder};

#[post("/signin")]
pub async fn signin_user() -> impl Responder {
    HttpResponse::Ok().body("sign in successfull")
}
