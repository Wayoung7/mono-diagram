use std::iter::repeat;

use pest::Parser;
use pest_derive::Parser;
use unicode_width::UnicodeWidthStr;

use crate::{
    data_structure::table::{Table, TableCell},
    utils::pad_string_right,
};

use super::Diagram;

const PALETTE: [char; 3] = ['+', '-', '|'];

#[derive(Default)]
pub struct TableDiagram {
    pub data: Table<String>,
}

impl Diagram for TableDiagram {
    fn parse_from_str(&mut self, input: &str) {
        let mut table_data = Table::<String>::default();
        let diagram = TableDiagramParser::parse(Rule::diagram, input)
            .unwrap()
            .next()
            .unwrap();
        let mut width: usize = 0;
        for (_idx, line) in diagram.into_inner().enumerate() {
            let mut row: Vec<TableCell<String>> = Vec::new();
            match line.as_rule() {
                Rule::line => {
                    for cell in line.into_inner() {
                        row.push(TableCell {
                            value: cell.as_str().to_owned(),
                        });
                    }
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
    }

    fn print(&self) {
        let mut col_width: Vec<usize> = repeat(0).take(self.data.width).collect();
        for row in self.data.cells.iter() {
            for (idx, col) in row.iter().enumerate() {
                let cur_width = UnicodeWidthStr::width(col.value.as_str());
                if cur_width > col_width[idx] {
                    col_width[idx] = cur_width;
                }
            }
        }
        let separating_line: String = col_width.iter().fold(PALETTE[0].to_string(), |acc, &w| {
            format!(
                "{}{}{}",
                acc,
                repeat(PALETTE[1]).take(w + 2).collect::<String>(),
                PALETTE[0]
            )
        });
        println!("{}", separating_line);
        for row in self.data.cells.iter() {
            let text_line: String =
                col_width
                    .iter()
                    .enumerate()
                    .fold(PALETTE[2].to_string(), |acc, (idx, &w)| {
                        let text_with_space = if idx < row.len() {
                            " ".to_string() + &row[idx].value
                        } else {
                            " ".to_string()
                        };
                        format!(
                            "{}{}{}",
                            acc,
                            pad_string_right(&text_with_space, w + 2, ' ',),
                            PALETTE[2]
                        )
                    });
            println!("{}", text_line);
            println!("{}", separating_line);
        }
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/table.pest"]
struct TableDiagramParser;
