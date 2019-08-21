#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::embed_migrations;

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Book, NewBook};
embed_migrations!("./migrations");

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error run migrations");

    return connection;
}

pub fn create_book<'a>(
    conn: &PgConnection,
    authors: &'a str,
    title: &'a str
) -> QueryResult<Book> {
    use schema::books;

    let new_book = NewBook {
        authors: authors,
        title: title,
    };

    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
}

pub fn get_book<'a>(
    conn: &PgConnection,
    id: i32
) -> std::result::Result<Book, diesel::result::Error> {
    use schema::books;

    return books::table.filter(books::id.eq(id)).first(conn);
}

pub fn update_book<'a>(
    conn: &PgConnection,
    id: i32,
    authors: &'a str,
    title: &'a str
) -> QueryResult<Book> {
    use schema::books;

    let book = Book {
        id: id,
        authors: authors.to_owned(),
        title: title.to_owned(),
    };

    diesel::update(
        books::table.filter(books::id.eq(id))
    ).set(&book).get_result(conn)
}