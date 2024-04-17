use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

use crate::data_structure::table::Table;

use super::Diagram;

#[derive(Default)]
pub struct GridDiagram {
    pub data: Table<String>,
    pub cell_width: usize,
}

impl Diagram for GridDiagram {
    fn parse_from_str(&mut self, input: &str) {
        let diagram = GridDiagrmParser::parse(Rule::diagram, input)
            .unwrap()
            .next()
            .unwrap();
        let mut grid_data: Table<String> = Table::default();
        let mut assign_map: HashMap<(usize, usize), &str> = HashMap::new();
        let mut max_cell_width = 0;
        for ele in diagram.into_inner() {
            match ele.as_rule() {
                Rule::size => {
                    let mut size_inner = ele.into_inner();
                    grid_data.width = size_inner.next().unwrap().as_str().parse().unwrap();
                    grid_data.height = size_inner.next().unwrap().as_str().parse().unwrap();
                }
                Rule::assign => {
                    let mut assign_inner = ele.into_inner();
                    let pos = assign_inner.next().unwrap();
                    let mut pos_inner = pos.into_inner();
                    let x = pos_inner.next().unwrap().as_str().parse().unwrap();
                    let y = pos_inner.next().unwrap().as_str().parse().unwrap();
                    let cell = assign_inner.next().unwrap().as_str();
                    if cell.len() > max_cell_width {
                        max_cell_width = cell.len();
                    }
                    assign_map.insert((x, y), cell);
                }
                _ => (),
            }
        }
        println!(
            "{} {}\n{:#?}",
            grid_data.width, grid_data.height, assign_map
        );
    }

    fn print(&self) {}
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/grid.pest"]
struct GridDiagrmParser;
