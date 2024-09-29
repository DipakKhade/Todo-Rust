use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, post, App, HttpResponse, HttpServer};
use Todo_Rust::signin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173/signin")
            // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            // .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            // .supports_credentials()
            .max_age(3600);
        App::new().wrap(cors).service(signin::signin_user)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
