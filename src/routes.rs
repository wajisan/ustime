
use actix_web::{web};
use crate::controllers;


pub fn init_routes(conf: &mut web::ServiceConfig) {
    conf.route("/", web::get().to(controllers::status::get_status) );
}