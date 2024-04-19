mod args;
mod data_structure;
mod diagram;
mod parser;
mod utils;
mod watch;

use std::io::stdout;

use args::{Cli, Commands};
use clap::Parser;
use crossterm::{cursor, execute, terminal};
use parser::{parse, write};
use utils::add_prefix;
use watch::watch;

fn main() {
    let cli = Cli::parse();
    let prefix = cli.prefix;
    match &cli.command {
        Commands::Build { file } => build_cmd(file),
        Commands::Print { file } => print_cmd(file, prefix),
        Commands::Watch { file } => watch_cmd(file, prefix),
        _ => (),
    }
    // watch_cmd("examples/binary_tree", None);
}

fn build_cmd(file: &str) {
    let res = parse(file);
    match res {
        Ok(_) => println!("Build success"),
        Err(e) => println!("{}", e),
    }
}

fn print_cmd(file: &str, prefix: Option<String>) {
    let result = parse(file).and_then(|d| write(&d));
    match result {
        Ok(d) => println!(
            "{}",
            add_prefix(
                String::from_utf8_lossy(&d).to_string(),
                &prefix.unwrap_or("".to_string())
            )
        ),
        Err(e) => println!("{}", e),
    }
}

fn watch_cmd(file: &str, prefix: Option<String>) {
    match watch(file, prefix) {
        Err(e) => {
            execute!(&mut stdout(), cursor::Show, terminal::LeaveAlternateScreen).unwrap();
            terminal::disable_raw_mode().unwrap();
            println!("{}", e);
        }
        _ => (),
    }
}
