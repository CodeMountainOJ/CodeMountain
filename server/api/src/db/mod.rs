pub mod schema;
pub mod user;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
                            .expect("Environment variable 'DATABASE_URL' must be set");

    PgConnection::establish(&database_url)
                    .expect("Failed to connect to db!")
}