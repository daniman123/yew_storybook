// #![allow(unused)]

use cli::command_handler::parse_arguments;

mod cli;
mod core;
mod error;
mod init;
mod prelude;
mod utils;

fn main() {
    let init_flag = parse_arguments();
    if init_flag {
        
    }
}
