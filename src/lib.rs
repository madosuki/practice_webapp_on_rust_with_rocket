#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connect_db() -> SqliteConnection {

    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("must be set url");

    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
