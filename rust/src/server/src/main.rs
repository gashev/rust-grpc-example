use futures::Future;
use grpcio::{Environment, ServerBuilder, UnarySink};
use std::{thread, time};
use std::sync::Arc;

#[path = "./../../../books.rs"]
mod books;

#[path = "./../../../books_grpc.rs"]
mod books_grpc;

use books_grpc::{create_books, Books};
use books::{AddBookRequest, BookReply, GetBookRequest};

use db;
use diesel::pg::PgConnection;

struct BooksService {
    connection: PgConnection,
}

impl Clone for BooksService {
    fn clone(&self) -> Self {
        BooksService {
            connection: db::establish_connection(),
        }
    }
}

impl Books for BooksService {
    fn add_book(
        &mut self, 
        _ctx: ::grpcio::RpcContext, 
        _req: AddBookRequest, 
        _sink: UnarySink<BookReply>
    ) {
        println!("add_book request");
        let mut resp = BookReply::default();
        let authors = _req.get_authors();
        let title = _req.get_title();
        let book = db::create_book(&self.connection, authors, title);
        resp.set_id(book.id);
        resp.set_authors(book.authors);
        resp.set_title(book.title);
        let f = _sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
        _ctx.spawn(f)
    }
    
    fn get_book(
        &mut self, 
        _ctx: ::grpcio::RpcContext, 
        _req: GetBookRequest, 
        _sink: UnarySink<BookReply>
    ) {
        println!("get_book request");
    }
}

fn sleep() {
    loop {
        thread::sleep(time::Duration::from_millis(1000));            
    };
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = create_books(BooksService {connection: db::establish_connection(),});
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    thread::spawn(sleep).join().unwrap();
}