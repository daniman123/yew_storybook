// #![allow(unused)]

use cli::command_handler::parse_arguments;

mod cli;
mod core;
mod error;
mod init;
mod prelude;
mod utils;

fn main() {
    parse_arguments();
}
