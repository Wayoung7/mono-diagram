mod data_structure;
mod diagram;
mod parser;
mod utils;

use parser::{parse, print};

fn main() {
    let res = parse("examples/binary_tree").unwrap();
    print(&res);
    // println!("{}", "🉑i".chars().);
}
