use std::{
    fs,
    io::{Read, Seek},
};

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::{
    attrib::Attrib,
    diagram::{
        binary_tree_diagram::BinaryTreeDiagram, dag_diagram::DagGraph, gantt_diagram::GanttDiagram,
        grid_diagram::GridDiagram, table_diagram::TableDiagram, timeline_diagram::TimelineDiagram,
        Diagram,
    },
};

/// Parse all diagrams in input file
pub fn parse(script_path: &str) -> Result<Vec<Box<dyn Diagram>>> {
    let mut parsed_diagrams: Vec<Box<dyn Diagram>> = Vec::new();
    let mut file_content = String::new();
    let mut file = fs::File::open(script_path)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.read_to_string(&mut file_content)?;
    let main = ScriptParser::parse(Rule::main, &file_content)
        .map_err(|e| Error::msg(format!("parsing error: {}", e.line())))?
        .next()
        .unwrap();
    for diagram in main.into_inner() {
        if diagram.as_rule() == Rule::diagram {
            let mut diagram_inner = diagram.into_inner();
            let mut d = init_diagram(
                diagram_inner
                    .next()
                    .unwrap()
                    .as_str()
                    .trim()
                    .trim_end_matches(']')
                    .trim_start_matches('[')
                    .trim(),
            );
            let mut attribs = Attrib::default();
            let mut content = "";
            for next in diagram_inner {
                if next.as_rule() == Rule::attribs {
                    attribs = Attrib::parse_from_str(next.as_str())?;
                } else if next.as_rule() == Rule::content {
                    content = next.as_str();
                }
            }
            d.parse_from_str(content, attribs)?;
            parsed_diagrams.push(d);
        }
    }
    Ok(parsed_diagrams)
}

/// Write output diagram to buffer
pub fn write(diagrams: &Vec<Box<dyn Diagram>>) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    for d in diagrams {
        buffer.append(&mut d.write().unwrap());
        buffer.push(b'\n');
    }
    Ok(buffer)
}

/// Initialization
fn init_diagram(title: &str) -> Box<dyn Diagram> {
    match title.to_ascii_lowercase().as_str() {
        "binary_tree" => Box::<BinaryTreeDiagram>::default(),
        "table" => Box::<TableDiagram>::default(),
        "grid" => Box::<GridDiagram>::default(),
        "dag" => Box::<DagGraph>::default(),
        "timeline" => Box::<TimelineDiagram>::default(),
        "gantt" => Box::<GanttDiagram>::default(),
        _ => unreachable!(),
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/script.pest"]
pub struct ScriptParser;
