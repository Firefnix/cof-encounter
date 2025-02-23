use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct Args {
    /// Activate verbose mode
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Edit an NPC
    Edit,
    /// List all items of a given type
    List,
}