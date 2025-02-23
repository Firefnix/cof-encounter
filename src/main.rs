mod args;
mod db;
mod display;
mod list;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    let conn = db::setup();
    match args.cmd {
        Some(args::Commands::Edit) => println!("Edit"),
        Some(args::Commands::List) => list::list(&conn),
        None => (),
    }
}
