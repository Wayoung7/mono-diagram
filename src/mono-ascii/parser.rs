use std::{
    fs,
    io::{Read, Result},
};

use pest::Parser;
use pest_derive::Parser;

use crate::diagram::{
    binary_tree_diagram::BinaryTreeDiagram, grid::GridDiagram, table::TableDiagram, Diagram,
};

pub fn parse(script_path: &str) -> Result<Vec<Box<dyn Diagram>>> {
    let mut parsed_diagrams: Vec<Box<dyn Diagram>> = Vec::new();
    let mut file_content = String::new();
    let mut file = fs::File::open(script_path).unwrap();
    file.read_to_string(&mut file_content)?;
    let main = ScriptParser::parse(Rule::main, &file_content)
        .unwrap()
        .next()
        .unwrap();
    for diagram in main.into_inner() {
        match diagram.as_rule() {
            Rule::diagram => {
                let mut diagram_inner = diagram.into_inner();
                let title = diagram_inner.next().unwrap().into_inner().as_str();
                let content = diagram_inner.next().unwrap().as_str();
                let mut diagram = init_diagram(title);
                diagram.parse_from_str(content);
                parsed_diagrams.push(diagram);
            }

            _ => (),
        }
    }
    Ok(parsed_diagrams)
}

pub fn print(diagrams: &Vec<Box<dyn Diagram>>) {
    diagrams.iter().for_each(|d| d.print());
}

fn init_diagram(title: &str) -> Box<dyn Diagram> {
    match title {
        "binary_tree" => Box::new(BinaryTreeDiagram::default()),
        "table" => Box::new(TableDiagram::default()),
        "grid" => Box::new(GridDiagram::default()),
        _ => panic!(""),
    }
}

#[derive(Parser)]
#[grammar = "mono-ascii/grammar/script.pest"]
pub struct ScriptParser;
