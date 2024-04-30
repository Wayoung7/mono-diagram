mod args;
mod data_structure;
mod diagram;
mod parser;
mod utils;

use args::Cli;
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use parser::{parse, write};
use utils::add_prefix;

fn main() {
    let cli = Cli::parse();
    let prefix = cli.prefix;
    let copy = cli.copy;
    let file = &cli.file_path;
    let result = parse(file).and_then(|d| write(&d));
    match result {
        Ok(d) => {
            println!(
                "{}",
                add_prefix(
                    String::from_utf8_lossy(&d).to_string(),
                    &prefix.clone().unwrap_or("".to_string())
                )
            );
            if copy {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(add_prefix(
                    String::from_utf8_lossy(&d).to_string(),
                    &prefix.unwrap_or("".to_string()),
                ))
                .unwrap();
            }
        }
        Err(e) => println!("{}", e),
    }
}
