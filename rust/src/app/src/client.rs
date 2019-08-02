use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

#[path = "./../../../books.rs"]
mod books;

#[path = "./../../../books_grpc.rs"]
mod books_grpc;

use books_grpc::BooksClient;
use books::{AddBookRequest, BookReply};
 
fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = BooksClient::new(ch);

    let mut req = AddBookRequest::default();
    req.set_authors("author".to_owned());
    req.set_title("title".to_owned());
    let reply: BookReply = client.add_book(&req).expect("rpc");
    println!("received: {}", reply.get_id());
}