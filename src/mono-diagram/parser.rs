use std::{
    fs,
    io::{Read, Seek},
};

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::diagram::{
    binary_tree_diagram::BinaryTreeDiagram, dag_diagram::DagGraph, grid_diagram::GridDiagram,
    table_diagram::TableDiagram, Diagram,
};

pub fn parse(script_path: &str) -> Result<Vec<Box<dyn Diagram>>> {
    let mut parsed_diagrams: Vec<Box<dyn Diagram>> = Vec::new();
    let mut file_content = String::new();
    let mut file = fs::File::open(script_path)?;
    file.read_to_string(&mut file_content)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    let main = ScriptParser::parse(Rule::main, &file_content)
        .map_err(|e| Error::msg(format!("parsing error: {}", e.line())))?
        .next()
        .unwrap();
    for diagram in main.into_inner() {
        if diagram.as_rule() == Rule::diagram {
            let mut diagram_inner = diagram.into_inner();
            let title = diagram_inner.next().unwrap().into_inner().as_str();
            let content = diagram_inner.next().unwrap().as_str();
            let mut diagram = init_diagram(title);
            diagram.parse_from_str(content)?;
            parsed_diagrams.push(diagram);
        }
    }
    Ok(parsed_diagrams)
}

pub fn write(diagrams: &Vec<Box<dyn Diagram>>) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    for d in diagrams {
        buffer.append(&mut d.write().unwrap());
        buffer.push(b'\n');
    }
    Ok(buffer)
}

fn init_diagram(title: &str) -> Box<dyn Diagram> {
    match title {
        "binary_tree" => Box::<BinaryTreeDiagram>::default(),
        "table" => Box::<TableDiagram>::default(),
        "grid" => Box::<GridDiagram>::default(),
        "dag" => Box::<DagGraph>::default(),
        _ => panic!(""),
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/script.pest"]
pub struct ScriptParser;
