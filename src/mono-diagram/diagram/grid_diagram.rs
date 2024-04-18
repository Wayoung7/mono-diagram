use std::{collections::HashMap, io::Write as _, iter::repeat};

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::{
    data_structure::table::{Table, TableCell},
    utils::pad_string_center,
};

use super::Diagram;

const PALETTE: [char; 3] = ['+', '-', '|'];
const MAX_CELL_WIDTH: usize = 3;

#[derive(Default)]
pub struct GridDiagram {
    pub data: Table<String>,
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
                    if cell.len() > MAX_CELL_WIDTH {
                        return Err(Error::msg(format!(
                            "diagram error: length of text in a grid cell should be less than {MAX_CELL_WIDTH}, context: {x},{y}:{cell}"
                        )));
                    }
                    if assign_map.contains_key(&(x, y)) {
                        return Err(Error::msg(format!(
                            "diagram error: assign a cell for multiple times, context: {x},{y}:{cell}"
                        )));
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
        self.data = grid_data;

        Ok(())
    }

    fn write(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let separating_line = (0..self.data.width).fold(PALETTE[0].to_string(), |acc, _| {
            format!(
                "{}{}{}",
                acc,
                repeat(PALETTE[1]).take(MAX_CELL_WIDTH).collect::<String>(),
                PALETTE[0]
            )
        });
        for row in self.data.cells.iter() {
            let text_line: String = (0..self.data.width).fold(PALETTE[2].to_string(), |acc, i| {
                format!(
                    "{}{}{}",
                    acc,
                    pad_string_center(&row[i].value, MAX_CELL_WIDTH, ' ', ' '),
                    PALETTE[2]
                )
            });
            writeln!(&mut buffer, "{}", separating_line)?;
            writeln!(&mut buffer, "{}", text_line)?;
        }
        writeln!(&mut buffer, "{}", separating_line)?;
        Ok(buffer)
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/grid.pest"]
struct GridDiagrmParser;
