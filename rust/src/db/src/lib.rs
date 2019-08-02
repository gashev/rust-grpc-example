#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Book, NewBook};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_book<'a>(conn: &PgConnection, authors: &'a str, title: &'a str) -> Book {
    use schema::books;

    let new_book = NewBook {
        authors: authors,
        title: title,
    };

    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error saving new book")
}