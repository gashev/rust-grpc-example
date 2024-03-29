mod cli;
mod commands;

fn main() {
    match cli::get_cli().get_matches().subcommand() {
        ("add", Some(sub_m)) => commands::add_book(&sub_m),
        ("get", Some(sub_m)) => commands::get_book(&sub_m),
        ("all", Some(sub_m)) => commands::get_all_books(&sub_m),
        ("update", Some(sub_m)) => commands::update_book(&sub_m),
        ("delete", Some(sub_m)) => commands::delete_book(&sub_m),
        _ => println!("Incorrect command.")
    }
}