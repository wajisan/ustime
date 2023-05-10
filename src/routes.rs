
use actix_web::{web};
use crate::controllers;
use crate::database;


pub fn init_routes(conf: &mut web::ServiceConfig) {
    conf.route("/", web::get().to(controllers::status::get_status) );
    /*let mut database_client = database::get_database_connection();
    for row in database_client.query("SELECT id FROM users").unwrap() {
        println!("{:?}", row.get(0))
    }*/

}