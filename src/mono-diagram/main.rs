mod data_structure;
mod diagram;
mod parser;
mod utils;

use parser::{parse, print};

fn main() {
    let res = parse("examples/table").unwrap();
    print(&res);
    // println!("|{:^8}|", "🉑我们iii".to_string());
}
