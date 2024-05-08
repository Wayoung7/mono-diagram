use std::{cmp::max, io::Write as _};

use anyhow::Error;
use pest::Parser;
use pest_derive::Parser;

use crate::attrib::{Attrib, Style};

use super::Diagram;

#[derive(Default)]
pub struct TimelineDiagram {
    data: Vec<(String, String)>,
    max_width: usize,
    attribs: Attrib,
}

impl Diagram for TimelineDiagram {
    fn parse_from_str(&mut self, input: &str, attribs: Attrib) -> anyhow::Result<()> {
        let mut timeline_data = Vec::new();
        let diagram = TimelineDiagramParser::parse(Rule::diagram, input)
            .map_err(|e| {
                Error::msg(format!(
                    "parsing error: incorrect timeline grammar, context: {}",
                    e.line()
                ))
            })?
            .next()
            .unwrap();
        for line in diagram.into_inner() {
            if line.as_rule() == Rule::line {
                let mut line_inner = line.into_inner();
                let time = line_inner.next().unwrap().as_str().to_string();
                if time.is_empty() {
                    return Err(Error::msg(
                        "parsing error: incorrect timeline grammar: time cannot be empty",
                    ));
                }
                self.max_width = max(self.max_width, time.len());
                let description = line_inner.next().unwrap().as_str().to_string();
                timeline_data.push((time, description));
            }
        }
        self.data = timeline_data;
        self.attribs = attribs;
        Ok(())
    }

    fn write(&self) -> anyhow::Result<Vec<u8>> {
        match self.attribs.style {
            Style::Ascii => self.write_ascii(),
            Style::Unicode => self.write_unicode(),
        }
    }
}

impl TimelineDiagram {
    fn write_ascii(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let max_left_dash_width = if self.max_width % 2 == 0 {
            self.max_width / 2
        } else {
            (self.max_width - 1) / 2
        } + 1;
        let mut empty_line = String::from_utf8(vec![b' '; max_left_dash_width]).unwrap();
        empty_line.push('|');

        // Beginning pipe
        for _ in 0..3 {
            writeln!(&mut buffer, "{}", empty_line)?;
        }
        for (time, des) in self.data.iter() {
            let len = time.len();
            let dashes_left = if len % 2 == 0 { len / 2 } else { (len - 1) / 2 } + 1;
            let shift_right = max_left_dash_width - dashes_left;
            // First line
            for _ in 0..shift_right {
                write!(&mut buffer, " ")?;
            }
            for _ in 0..dashes_left {
                write!(&mut buffer, "-")?;
            }
            write!(&mut buffer, "v")?;
            for _ in 0..dashes_left {
                write!(&mut buffer, "-")?;
            }
            writeln!(&mut buffer)?;

            // Middle line
            for _ in 0..shift_right {
                write!(&mut buffer, " ")?;
            }
            write!(&mut buffer, " ")?;
            if len % 2 == 0 {
                write!(&mut buffer, " ")?;
            }
            write!(&mut buffer, "{}", time)?;
            if !des.is_empty() {
                write!(&mut buffer, "  >--- ")?;
            }
            writeln!(&mut buffer, "{}", des)?;

            // Bottom line
            for _ in 0..shift_right {
                write!(&mut buffer, " ")?;
            }
            for _ in 0..dashes_left {
                write!(&mut buffer, "-")?;
            }
            write!(&mut buffer, "v")?;
            for _ in 0..dashes_left {
                write!(&mut buffer, "-")?;
            }
            writeln!(&mut buffer)?;

            // Pipe
            writeln!(&mut buffer, "{}", empty_line)?;
        }

        // Arrow at the end
        writeln!(&mut buffer, "{}", empty_line)?;
        writeln!(&mut buffer, "{}", empty_line)?;
        for _ in 0..max_left_dash_width {
            write!(&mut buffer, " ")?;
        }
        writeln!(&mut buffer, "V")?;

        Ok(buffer)
    }

    fn write_unicode(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let max_left_dash_width = if self.max_width % 2 == 0 {
            self.max_width / 2
        } else {
            (self.max_width - 1) / 2
        } + 1;
        let mut empty_line = String::from_utf8(vec![b' '; max_left_dash_width]).unwrap();
        empty_line.push('║');

        // Beginning pipe
        for _ in 0..3 {
            writeln!(&mut buffer, "{}", empty_line)?;
        }
        for (time, des) in self.data.iter() {
            let len = time.len();
            let dashes_left = if len % 2 == 0 { len / 2 } else { (len - 1) / 2 } + 1;
            let shift_right = max_left_dash_width - dashes_left;
            // First line
            for _ in 0..shift_right {
                write!(&mut buffer, " ")?;
            }
            for _ in 0..dashes_left {
                write!(&mut buffer, " ")?;
            }
            write!(&mut buffer, "╨")?;
            writeln!(&mut buffer)?;

            // Middle line
            for _ in 0..shift_right {
                write!(&mut buffer, " ")?;
            }
            write!(&mut buffer, " ")?;
            if len % 2 == 0 {
                write!(&mut buffer, " ")?;
            }
            write!(&mut buffer, "{}", time)?;
            if !des.is_empty() {
                write!(&mut buffer, "  ┄┄┄┄ ")?;
            }
            writeln!(&mut buffer, "{}", des)?;

            // Bottom line
            for _ in 0..shift_right {
                write!(&mut buffer, " ")?;
            }
            for _ in 0..dashes_left {
                write!(&mut buffer, " ")?;
            }
            write!(&mut buffer, "╥")?;
            writeln!(&mut buffer)?;

            // Pipe
            writeln!(&mut buffer, "{}", empty_line)?;
        }

        // Arrow at the end
        writeln!(&mut buffer, "{}", empty_line)?;
        writeln!(&mut buffer, "{}", empty_line)?;
        for _ in 0..max_left_dash_width {
            write!(&mut buffer, " ")?;
        }
        writeln!(&mut buffer, "▼")?;

        Ok(buffer)
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/timeline.pest"]
pub struct TimelineDiagramParser;
