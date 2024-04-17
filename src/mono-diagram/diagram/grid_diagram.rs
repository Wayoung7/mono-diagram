use std::collections::HashMap;

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::data_structure::table::{Table, TableCell};

use super::Diagram;

#[derive(Default)]
pub struct GridDiagram {
    pub data: Table<String>,
    pub cell_width: usize,
}

impl Diagram for GridDiagram {
    fn parse_from_str(&mut self, input: &str) -> Result<()> {
        let diagram = GridDiagrmParser::parse(Rule::diagram, input)
            .map_err(|e| {
                Error::msg(format!(
                    "parsing error: incorrect grid grammar, context: {}",
                    e.line()
                ))
            })?
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
                    if cell.len() > 3 {
                        return Err(Error::msg(format!(
                            "diagram error: length of text in a grid cell should be less than 3, context: {x},{y}:{cell}"
                        )));
                    }
                    if assign_map.contains_key(&(x, y)) {
                        return Err(Error::msg(format!(
                            "diagram error: assign a cell for multiple times, context: {x},{y}:{cell}"
                        )));
                    }
                    if cell.len() > max_cell_width {
                        max_cell_width = cell.len();
                    }
                    assign_map.insert((x, y), cell);
                }
                _ => (),
            }
        }
        for j in 1..=(grid_data.height) {
            let mut row: Vec<TableCell<String>> = Vec::new();
            for i in 1..=(grid_data.width) {
                if assign_map.contains_key(&(i, j)) {
                    row.push(TableCell {
                        value: assign_map[&(i, j)].to_string(),
                    });
                } else {
                    row.push(TableCell {
                        value: " ".to_string(),
                    });
                }
            }
            grid_data.cells.push(row);
        }
        self.cell_width = max_cell_width;
        self.data = grid_data;

        Ok(())
    }

    fn print(&self) {}
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/grid.pest"]
struct GridDiagrmParser;
