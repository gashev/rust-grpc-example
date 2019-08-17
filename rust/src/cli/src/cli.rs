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

fn update_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("update")
        .about("update book")
        .arg(Arg::with_name("id")
            .short("i")
            .long("id")
            .help("id")
            .required(true)
            .takes_value(true))
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

fn delete_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("delete")
        .about("delete book")
        .arg(Arg::with_name("id")
            .short("i")
            .long("id")
            .help("id")
            .required(true)
            .takes_value(true));
}

pub fn get_cli() -> clap::App<'static, 'static> {
    return
        App::new("cli")
        .subcommand(add_book_command())
        .subcommand(get_book_command())
        .subcommand(get_books_command())
        .subcommand(update_book_command())
        .subcommand(delete_book_command());
}