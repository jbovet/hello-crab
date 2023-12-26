use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello crustaceans!")
}

async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("this is fine!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/healthz", web::get().to(healthz))
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
