mod config;
mod models;
mod controllers;
mod routes;
mod database;

use actix_web::{HttpServer, App};
use std::io;
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();
    let config = crate::config::AppConfig::from_env().unwrap();
    println!("Server launch at address {} on port {}", config.server.host, config.server.port);

    HttpServer::new(|| {
        
        App::new()
        .configure(routes::init_routes)
    
    })
    .bind(format!("{}:{}", config.server.host, config.server.port).to_string())?
    .run()
    .await
}
