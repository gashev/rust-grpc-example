extern crate clap;
use clap::{Arg, App, SubCommand};

macro_rules! argument {
    ($name: expr, $short: expr, $long: expr, $help: expr) => {
        Arg::with_name($name)
        .short($short)
        .long($long)
        .help($help)
        .required(true)
        .takes_value(true)
    }
}

fn get_authors_argument() -> clap::Arg<'static, 'static> {
    argument!("authors", "a", "authors", "authors")
}

fn get_id_argument() -> clap::Arg<'static, 'static> {
    argument!("id", "i", "id", "id")
}

fn get_title_argument() -> clap::Arg<'static, 'static> {
    argument!("title", "t", "title", "title")
}

fn add_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("add")
        .about("add book")
        .arg(get_authors_argument())
        .arg(get_title_argument());
}

fn get_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("get")
        .about("get book")
        .arg(get_id_argument());
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
        .arg(get_id_argument())
        .arg(get_authors_argument())
        .arg(get_title_argument());
}

fn delete_book_command() -> clap::App<'static, 'static> {
    return
        SubCommand::with_name("delete")
        .about("delete book")
        .arg(get_id_argument());
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