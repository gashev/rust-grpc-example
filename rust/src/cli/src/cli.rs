extern crate clap;
use clap::{Arg, App, SubCommand};

fn add_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("add")
        .about("add book")
        .arg(Arg::with_name("authors")
            .short("a")
            .long("authors")
            .help("authors")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .help("title")
            .required(true)
            .takes_value(true));
}

fn get_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("get")
        .about("get book")
        .arg(Arg::with_name("id")
            .short("i")
            .long("id")
            .help("id")
            .required(true)
            .takes_value(true));
}

fn get_books_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("all")
        .about("get all books");
}

pub fn get_cli() -> clap::App<'static, 'static> {
    return
        App::new("cli")
        .subcommand(add_book_command())
        .subcommand(get_book_command())
        .subcommand(get_books_command());
}