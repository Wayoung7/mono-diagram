use anyhow::Result;

use crate::attrib::Attrib;

pub mod binary_tree_diagram;
pub mod dag_diagram;
pub mod gantt_diagram;
pub mod grid_diagram;
pub mod table_diagram;
pub mod timeline_diagram;

/// Abstract data type for a diagram
pub trait Diagram {
    fn parse_from_str(&mut self, input: &str, attribs: Attrib) -> Result<()>;
    fn write(&self) -> Result<Vec<u8>>;
}
