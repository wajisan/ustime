mod config;
mod models;


use models::Status;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder {
    println!("OH, CONNECTION");
    web::HttpResponse::Ok()
    .json(Status { status: "UP".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();
    let config = crate::config::AppConfig::from_env().unwrap();

    println!("{:?}", config.server);
    //println!("Server launch at address {} on port {}", config.server.host, config.server.port);

    HttpServer::new(|| {
        
        App::new()
        .route("/", web::get().to(status))
    
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
