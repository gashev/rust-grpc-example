use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

#[path = "../../books.rs"]
mod books;

#[path = "../../books_grpc.rs"]
mod books_grpc;

use books_grpc::{create_books, Books};
use books::{AddBookRequest, BookReply, GetBookRequest};

#[derive(Clone)]
struct BooksService;

impl Books for BooksService {
    fn add_book(&mut self, ctx: ::grpcio::RpcContext, req: AddBookRequest, sink: UnarySink<BookReply>) {
        println!("add_book request");
    }
    
    fn get_book(&mut self, ctx: ::grpcio::RpcContext, req: GetBookRequest, sink: UnarySink<BookReply>) {
        println!("get_book request");
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = create_books(BooksService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}