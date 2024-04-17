mod data_structure;
mod diagram;
mod parser;
mod utils;

use parser::{parse, print};

fn main() {
    let res = parse("examples/grid").unwrap();
    print(&res);
}
