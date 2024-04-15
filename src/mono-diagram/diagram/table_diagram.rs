use pest::Parser;
use pest_derive::Parser;

use crate::data_structure::table::Table;

use super::Diagram;

#[derive(Default)]
pub struct TableDiagram {
    pub data: Table<String>,
}

impl Diagram for TableDiagram {
    fn parse_from_str(&mut self, input: &str) {
        let diagram = TableDiagramParser::parse(Rule::diagram, input)
            .unwrap()
            .next()
            .unwrap();
        println!("{:#?}", diagram);
    }

    fn print(&self) {}
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/table.pest"]
struct TableDiagramParser;
