use std::{cmp::max, io::Write as _};

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::{
    attrib::{Attrib, Style},
    utils::{pad_string_center, pad_string_left},
};

use super::Diagram;

#[derive(Default)]
pub struct GanttDiagram {
    timeline: Vec<String>,
    period: Vec<(String, f32, f32)>,
    time_width: usize,
    task_width: usize,
    attribs: Attrib,
}

impl Diagram for GanttDiagram {
    fn parse_from_str(&mut self, input: &str, attribs: Attrib) -> anyhow::Result<()> {
        let mut timeline_data = Vec::new();
        let mut period_data = Vec::new();
        let diagram = GanttDiagramParser::parse(Rule::diagram, input)
            .map_err(|e| {
                Error::msg(format!(
                    "parsing error: incorrect gantt grammar, context: {}",
                    e.line()
                ))
            })?
            .next()
            .unwrap();
        for line in diagram.into_inner() {
            match line.as_rule() {
                Rule::timeline => {
                    let timeline_inner = line.into_inner();
                    for time in timeline_inner {
                        let time_string = time.as_str().to_string();
                        self.time_width = max(self.time_width, time_string.len());
                        timeline_data.push(time_string);
                    }
                }
                Rule::period => {
                    let mut period_inner = line.into_inner();
                    let task = period_inner.next().unwrap().as_str().to_string();
                    self.task_width = max(self.task_width, task.len());
                    let start = period_inner
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<f32>()
                        .map_err(|e| Error::msg(format!("parsing error: {}", e)))?;
                    let end = period_inner
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<f32>()
                        .map_err(|e| Error::msg(format!("parsing error: {}", e)))?;
                    if start >= end {
                        return Err(Error::msg(format!("diagram error: task ending time must be larger than starting time: task: {}, start: {}, end: {}", task, start, end)));
                    }
                    period_data.push((task, start, end));
                }
                _ => (),
            }
        }
        for (task, _, ed) in period_data.iter() {
            if *ed > timeline_data.len() as f32 {
                return Err(Error::msg(format!("diagram error: task ending time exceeds timeline: task: {}, end: {}, timeline length: {}", task, ed, timeline_data.len())));
            }
        }
        self.timeline = timeline_data;
        self.period = period_data;
        self.attribs = attribs;
        Ok(())
    }

    fn write(&self) -> Result<Vec<u8>> {
        const PALETTE_ASCII: [char; 8] = ['|', '-', '+', '|', '<', '=', '>', '.'];
        const PALETTE_UNICODE: [char; 8] = [' ', '─', '─', '│', '[', '━', ']', '·'];
        let palette = match self.attribs.style {
            Style::Ascii => PALETTE_ASCII,
            Style::Unicode => PALETTE_UNICODE,
        };

        let mut buffer = Vec::new();
        let time_width = max(get_time_width(self.timeline.len()), self.time_width);

        // Write first line
        for _ in 0..self.task_width + 2 {
            write!(&mut buffer, " ")?;
        }
        for time in self.timeline.iter() {
            write!(&mut buffer, "{}", palette[0])?;
            write!(
                &mut buffer,
                "{}",
                pad_string_center(time, time_width, ' ', ' ')
            )?;
        }
        writeln!(&mut buffer)?;

        // Write middle line
        for _ in 0..self.task_width + 2 {
            write!(&mut buffer, "{}", palette[1])?;
        }
        for _ in self.timeline.iter() {
            write!(&mut buffer, "{}", palette[2])?;
            for _ in 0..time_width {
                write!(&mut buffer, "{}", palette[1])?;
            }
        }
        writeln!(&mut buffer, "{}", palette[1])?;

        // Write tasks
        for (task, st, ed) in self.period.iter() {
            let st_ = (st * (time_width + 1) as f32) as usize;
            let ed_ = (ed * (time_width + 1) as f32) as usize;
            write!(
                &mut buffer,
                "{} {}",
                pad_string_left(task, self.task_width + 1, ' '),
                palette[3]
            )?;
            for i in 0..st_ {
                if (i + 1) % (time_width + 1) == 0 {
                    write!(&mut buffer, "{}", palette[7])?;
                } else {
                    write!(&mut buffer, " ")?;
                }
            }
            write!(&mut buffer, "{}", palette[4])?;
            for _ in st_ + 1..=ed_ - 1 {
                write!(&mut buffer, "{}", palette[5])?;
            }
            write!(&mut buffer, "{}", palette[6])?;
            for i in ed_ + 1..(time_width + 1) * (self.timeline.len() - 1) {
                if (i + 1) % (time_width + 1) == 0 {
                    write!(&mut buffer, "{}", palette[7])?;
                } else {
                    write!(&mut buffer, " ")?;
                }
            }
            writeln!(&mut buffer)?;
        }
        for _ in 0..self.task_width + 2 {
            write!(&mut buffer, " ")?;
        }
        writeln!(&mut buffer, "{}", palette[3])?;

        Ok(buffer)
    }
}

fn get_time_width(cnt: usize) -> usize {
    if cnt <= 2 {
        17
    } else if cnt == 3 {
        14
    } else if cnt == 4 {
        12
    } else if cnt == 5 {
        10
    } else if cnt == 6 {
        8
    } else if cnt == 7 {
        7
    } else if cnt == 8 {
        6
    } else if cnt == 9 || cnt == 10 {
        5
    } else {
        4
    }
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/gantt.pest"]
pub struct GanttDiagramParser;
