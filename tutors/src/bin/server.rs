use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure Routes
fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. TutorFront is alive and kicking")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct the app and configure routes
    let app = move || App::new().configure(general_routes);
    // Starting the HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
