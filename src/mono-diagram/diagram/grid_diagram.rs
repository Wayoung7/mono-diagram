use std::{collections::HashMap, io::Write as _, iter::repeat};

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::{
    attrib::{Attrib, Style},
    data_structure::table::{Table, TableCell},
    utils::pad_string_center,
};

use super::Diagram;

const MAX_CELL_WIDTH: usize = 3;

#[derive(Default)]
pub struct GridDiagram {
    data: Table<String>,
    attribs: Attrib,
}

impl Diagram for GridDiagram {
    fn parse_from_str(&mut self, input: &str, attribs: Attrib) -> Result<()> {
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
                Rule::width => {
                    grid_data.width = ele.into_inner().next().unwrap().as_str().parse().unwrap();
                }
                Rule::height => {
                    grid_data.height = ele.into_inner().next().unwrap().as_str().parse().unwrap();
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
        if grid_data.width == 0 || grid_data.height == 0 {
            return Err(Error::msg(
                "diagram error: please specify the width and height of the grid",
            ));
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
        self.attribs = attribs;
        Ok(())
    }

    fn write(&self) -> Result<Vec<u8>> {
        const PALETTE_ASCII: [char; 11] = ['+', '+', '+', '+', '-', '|', '+', '+', '+', '+', '+'];
        const PALETTE_UNICODE: [char; 11] = ['┌', '┐', '└', '┘', '─', '│', '┬', '┴', '├', '┤', '┼'];
        let palette = match self.attribs.style {
            Style::Ascii => PALETTE_ASCII,
            Style::Unicode => PALETTE_UNICODE,
        };

        let separating_line = (0..self.data.width - 1).fold(
            repeat(palette[4]).take(MAX_CELL_WIDTH).collect::<String>(),
            |acc, _| {
                format!(
                    "{}{}{}",
                    acc,
                    palette[10],
                    repeat(palette[4]).take(MAX_CELL_WIDTH).collect::<String>(),
                )
            },
        );
        let first_line = (0..self.data.width - 1).fold(
            repeat(palette[4]).take(MAX_CELL_WIDTH).collect::<String>(),
            |acc, _| {
                format!(
                    "{}{}{}",
                    acc,
                    palette[6],
                    repeat(palette[4]).take(MAX_CELL_WIDTH).collect::<String>(),
                )
            },
        );
        let last_line = (0..self.data.width - 1).fold(
            repeat(palette[4]).take(MAX_CELL_WIDTH).collect::<String>(),
            |acc, _| {
                format!(
                    "{}{}{}",
                    acc,
                    palette[7],
                    repeat(palette[4]).take(MAX_CELL_WIDTH).collect::<String>(),
                )
            },
        );

        let mut buffer = Vec::new();
        for (idx, row) in self.data.cells.iter().enumerate() {
            let text_line: String = (0..self.data.width).fold(palette[5].to_string(), |acc, i| {
                format!(
                    "{}{}{}",
                    acc,
                    pad_string_center(&row[i].value, MAX_CELL_WIDTH, ' ', ' '),
                    palette[5]
                )
            });
            if idx == 0 {
                writeln!(&mut buffer, "{}{}{}", palette[0], first_line, palette[1])?;
            } else {
                writeln!(
                    &mut buffer,
                    "{}{}{}",
                    palette[8], separating_line, palette[9]
                )?;
            }
            writeln!(&mut buffer, "{}", text_line)?;
        }
        writeln!(&mut buffer, "{}{}{}", palette[2], last_line, palette[3])?;
        Ok(buffer)
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/grid.pest"]
struct GridDiagrmParser;
