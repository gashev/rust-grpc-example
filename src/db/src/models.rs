use super::schema::books;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub authors: String,
    pub title: String,
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a> {
    pub authors: &'a str,
    pub title: &'a str,
}