use parser::parse;

mod data_structure;
mod diagram;
mod parser;

fn main() {
    let res = parse("examples/binary_tree").unwrap();
}
