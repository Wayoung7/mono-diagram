use std::{io::Write as _, iter::repeat};

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::{
    attrib::{Attrib, Style},
    data_structure::table::{Table, TableCell},
    utils::pad_string_right,
};

use super::Diagram;

#[derive(Default)]
pub struct TableDiagram {
    data: Table<String>,
    attribs: Attrib,
}

impl Diagram for TableDiagram {
    fn parse_from_str(&mut self, input: &str, attribs: Attrib) -> Result<()> {
        let mut table_data = Table::<String>::default();
        let diagram = TableDiagramParser::parse(Rule::diagram, input)
            .map_err(|e| {
                Error::msg(format!(
                    "parsing error: incorrect table grammar, context: {}",
                    e.line()
                ))
            })?
            .next()
            .unwrap();
        let mut width: usize = 0;
        for (idx, line) in diagram.into_inner().enumerate() {
            let mut row: Vec<TableCell<String>> = Vec::new();
            match line.as_rule() {
                Rule::line => {
                    for cell in line.into_inner() {
                        row.push(TableCell {
                            value: cell.as_str().to_owned(),
                        });
                    }
                    table_data.height = idx + 1;
                }
                Rule::EOI => break,
                _ => (),
            }
            if row.len() > width {
                width = row.len();
            }
            table_data.cells.push(row);
        }

        table_data.width = width;
        self.data = table_data;
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

        let mut col_width: Vec<usize> = repeat(0).take(self.data.width).collect();
        for row in self.data.cells.iter() {
            for (idx, col) in row.iter().enumerate() {
                let cur_width = col.value.as_str().len();
                if cur_width > col_width[idx] {
                    col_width[idx] = cur_width;
                }
            }
        }
        let mut separating_line: String =
            col_width.iter().fold(palette[8].to_string(), |acc, &w| {
                format!(
                    "{}{}{}",
                    acc,
                    repeat(palette[4]).take(w + 2).collect::<String>(),
                    palette[10]
                )
            });
        separating_line.pop();
        separating_line.push(palette[9]);
        let mut first_line: String = col_width.iter().fold(palette[0].to_string(), |acc, &w| {
            format!(
                "{}{}{}",
                acc,
                repeat(palette[4]).take(w + 2).collect::<String>(),
                palette[6]
            )
        });
        first_line.pop();
        first_line.push(palette[1]);
        let mut last_line: String = col_width.iter().fold(palette[2].to_string(), |acc, &w| {
            format!(
                "{}{}{}",
                acc,
                repeat(palette[4]).take(w + 2).collect::<String>(),
                palette[7]
            )
        });
        last_line.pop();
        last_line.push(palette[3]);

        let mut buffer = Vec::new();
        for (idx, row) in self.data.cells.iter().enumerate() {
            let text_line: String =
                col_width
                    .iter()
                    .enumerate()
                    .fold(palette[5].to_string(), |acc, (idx, &w)| {
                        let text_with_space = if idx < row.len() {
                            " ".to_string() + &row[idx].value
                        } else {
                            " ".to_string()
                        };
                        format!(
                            "{}{}{}",
                            acc,
                            pad_string_right(&text_with_space, w + 2, ' ',),
                            palette[5]
                        )
                    });
            if idx == 0 {
                writeln!(&mut buffer, "{}", first_line)?;
            } else {
                writeln!(&mut buffer, "{}", separating_line)?;
            }
            writeln!(&mut buffer, "{}", text_line)?;
        }
        writeln!(&mut buffer, "{}", last_line)?;
        Ok(buffer)
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/table.pest"]
struct TableDiagramParser;
