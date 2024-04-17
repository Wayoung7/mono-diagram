mod data_structure;
mod diagram;
mod parser;
mod utils;

use parser::{parse, print};

fn main() {
    let res = parse("examples/grid");
    match res {
        Ok(r) => print(&r),
        Err(e) => println!("{}", e),
    }
    // print(&res);
}
