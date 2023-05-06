
use actix_web::{ web, Responder };
use crate::models;

pub async fn get_status() -> impl Responder {
    println!("OH, CONNECTION");
    web::HttpResponse::Ok().json(models::Status { status: "UP".to_string()})
}