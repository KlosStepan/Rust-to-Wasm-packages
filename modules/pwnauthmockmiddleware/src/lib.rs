// lib.rs

use actix_service::Service;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    // Add more claims as needed
}

fn generate_jwt_token() -> Result<String, Error> {
    let claims = Claims {
        sub: "dummy_user".to_owned(), // Replace with actual user info
    };

    let encoding_key = EncodingKey::from_secret("your-secret-key".as_ref()); // Replace with your secret key
    let token = encode(&Header::default(), &claims, &encoding_key)?;

    Ok(token)
}

async fn login() -> Result<HttpResponse, Error> {
    let token = generate_jwt_token()?;
    Ok(HttpResponse::Ok().body(token))
}

async fn logout() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Logged out successfully"))
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/login", web::get().to(login))
            .route("/logout", web::get().to(logout))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
