use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

#[path = "./../../../books.rs"]
mod books;

#[path = "./../../../books_grpc.rs"]
mod books_grpc;

use books_grpc::BooksClient;
use books::{
    AddBookRequest,
    BookReply,
    BooksReply,
    GetBookRequest,
    GetBooksRequest,
};

mod cli;

fn get_books_client() -> BooksClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("server:50051");
    return BooksClient::new(ch);
}

fn add_book(matches: &clap::ArgMatches<'static>) {
    let authors = matches.value_of("authors").unwrap();
    let title = matches.value_of("title").unwrap();

    println!("{} {}", authors, title);

    let mut req = AddBookRequest::default();
    req.set_authors(authors.to_owned());
    req.set_title(title.to_owned());

    let client = get_books_client();
    let reply: BookReply = client.add_book(&req).expect("rpc");
    println!("received: {}", reply.get_id());
}

fn get_book(matches: &clap::ArgMatches<'static>) {
    let id = matches.value_of("id").unwrap();

    println!("{}", id);

    let mut req = GetBookRequest::default();
    req.set_id(id.parse::<i32>().unwrap());

    let client = get_books_client();
    let reply: BookReply = client.get_book(&req).expect("rpc");
    println!(
        "received: {} {} {}",
        reply.get_id(),
        reply.get_authors(),
        reply.get_title()
    );
}

fn get_all_books(_matches: &clap::ArgMatches<'static>) {
    let req = GetBooksRequest::default();
    let client = get_books_client();
    let reply: BooksReply = client.get_books(&req).expect("rpc");
    println!("{:?}", reply);
}

fn main() {
    let matches = cli::get_cli().get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => add_book(&sub_m),
        ("get", Some(sub_m)) => get_book(&sub_m),
        ("all", Some(sub_m)) => get_all_books(&sub_m),
        _ => println!("Incorrect command.")
    }
}