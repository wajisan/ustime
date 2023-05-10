use postgres::{NoTls, Client};
use dotenv::dotenv;
use crate::config::AppConfig;

pub fn get_database_connection() -> Client {
    dotenv().ok();
    let config = AppConfig::from_env().unwrap();

    let user = config.database.user;
    let password = config.database.password;
    let host = config.database.host;
    let database = config.database.name;

    let database_url = format!("postgresql://{}:{}@{}/{}", user, password, host, database);
    let client = Client::connect(&database_url, NoTls).unwrap();
    client
}