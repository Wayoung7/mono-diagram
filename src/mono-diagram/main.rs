mod args;
mod data_structure;
mod diagram;
mod parser;
mod utils;
mod watch;

use std::io::stdout;

use args::{Cli, Commands};
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
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
        // Commands::Watch { file } => watch_cmd(file, prefix),
        Commands::Copy { file } => copy_cmd(file, prefix),
        _ => (),
    }
}

fn build_cmd(file: &str) {
    let res = parse(file);
    match res {
        Ok(_) => println!("Building succeed"),
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

fn copy_cmd(file: &str, prefix: Option<String>) {
    let result = parse(file).and_then(|d| write(&d));
    match result {
        Ok(d) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(add_prefix(
                String::from_utf8_lossy(&d).to_string(),
                &prefix.unwrap_or("".to_string()),
            ))
            .unwrap();
        }
        Err(e) => println!("{}", e),
    }
}
