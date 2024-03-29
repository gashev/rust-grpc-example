use futures::Future;
use grpcio::{
    Environment,
    RpcStatus,
    RpcStatusCode,
    ServerBuilder,
    UnarySink,
};
use std::{thread, time};
use std::sync::Arc;

#[path = "./../../../books.rs"]
mod books;

#[path = "./../../../books_grpc.rs"]
mod books_grpc;

use books_grpc::{create_books, Books};
use books::{
    AddBookRequest,
    BookReply,
    BooksReply,
    DeleteBookReply,
    DeleteBookRequest,
    GetBookRequest,
    GetBooksRequest,
    UpdateBookRequest,
};

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
        let result = db::create_book(&self.connection, authors, title);
        match result {
            Ok(book) => {
                resp.set_id(book.id);
                resp.set_authors(book.authors);
                resp.set_title(book.title);
                let f = _sink
                    .success(resp)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f)
            },
            Err(_e) => {
                let f = _sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unknown,
                        Some(_e.to_string()),
                    ))
                .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f);
            }
        }
    }

    fn get_book(
        &mut self,
        _ctx: ::grpcio::RpcContext,
        _req: GetBookRequest,
        _sink: UnarySink<BookReply>
    ) {
        println!("get_book request");
        let mut resp = BookReply::default();
        let id = _req.get_id();
        let result = db::get_book(&self.connection, id);
        match result {
            Ok(book) => {
                resp.set_id(book.id);
                resp.set_authors(book.authors);
                resp.set_title(book.title);
                let f = _sink
                    .success(resp)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f)
            },
            Err(_e) => {
                let f = _sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unknown,
                        Some(_e.to_string()),
                    ))
                .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f);
            }
        }
    }

    fn get_books(
        &mut self,
        _ctx: ::grpcio::RpcContext,
        _req: GetBooksRequest,
        _sink: UnarySink<BooksReply>
    ) {
        println!("get_books request");
        let mut resp = BooksReply::default();
        let result = db::get_books(&self.connection);

        match result {
            Ok(books) => {
                for x in &books {
                    let mut book = BookReply::default();
                    book.set_id(x.id);
                    book.set_authors(x.authors.to_owned());
                    book.set_title(x.title.to_owned());
                    resp.items.push(book);
                }

                let f = _sink
                    .success(resp)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f)
            },
            Err(_e) => {
                let f = _sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unknown,
                        Some(_e.to_string()),
                    ))
                .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f);
            }
        }
    }

    fn update_book(
        &mut self,
        _ctx: ::grpcio::RpcContext,
        _req: UpdateBookRequest,
        _sink: UnarySink<BookReply>
    ) {
        println!("update_book request");
        let mut resp = BookReply::default();
        let id = _req.get_id();
        let authors = _req.get_authors();
        let title = _req.get_title();
        let result = db::update_book(&self.connection, id, authors, title);
        match result {
            Ok(book) => {
                resp.set_id(book.id);
                resp.set_authors(book.authors);
                resp.set_title(book.title);
                let f = _sink
                    .success(resp)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f)
            },
            Err(_e) => {
                let f = _sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unknown,
                        Some(_e.to_string()),
                    ))
                .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f);
            }
        }
    }

    fn delete_book(
        &mut self,
        _ctx: ::grpcio::RpcContext,
        _req: DeleteBookRequest,
        _sink: UnarySink<DeleteBookReply>
    ) {
        println!("delete_book request");
        let mut resp = DeleteBookReply::default();
        let id = _req.get_id();
        let result = db::delete_book(&self.connection, id);
        match result {
            Ok(count) => {
                resp.set_deleted(count > 0);
                let f = _sink
                    .success(resp)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f)
            },
            Err(_e) => {
                let f = _sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unknown,
                        Some(_e.to_string()),
                    ))
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", _req, e));
                _ctx.spawn(f);
            }
        }
        }
}

fn sleep() {
    loop {
        thread::sleep(time::Duration::from_millis(1000));
    };
}

fn main() {
    let book_service = BooksService {
        connection: db::establish_connection(),
    };
    let service = create_books(book_service);
    let env = Arc::new(Environment::new(1));
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