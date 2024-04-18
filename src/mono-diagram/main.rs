mod args;
mod data_structure;
mod diagram;
mod parser;
mod utils;

use args::{Cli, Commands};
use clap::Parser;
use parser::{parse, write};
use utils::add_prefix;

fn main() {
    let cli = Cli::parse();
    let prefix = cli.prefix;
    match &cli.command {
        Commands::Build { file } => build_cmd(file),
        Commands::Print { file } => print_cmd(file, prefix),
        _ => (),
    }
    // let res = parse("examples/grid");
    // match res {
    //     Ok(r) => print(&r),
    //     Err(e) => println!("{}", e),
    // }
    // print(&res);
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
