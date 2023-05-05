use postgres::{NoTls, Client};
use dotenv::dotenv;

pub fn get_database_connection() -> Client {
    dotenv().ok();

    let user = env::var("PG_USER").unwrap();
    let password = env::var("PG_PASSWORD").unwrap();
    let host = env::var("PG_HOST").unwrap();
    let database = env::var("PG_DATABASE").unwrap();

    let database_url = format!("postgresql://{}:{}@{}/{}", user, password, host, database);
    let client = Client::connect(&database_url, NoTls).unwrap();
}