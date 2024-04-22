use anyhow::Result;

pub mod binary_tree_diagram;
pub mod dag_diagram;
pub mod grid_diagram;
pub mod table_diagram;

pub trait Diagram {
    fn parse_from_str(&mut self, input: &str) -> Result<()>;
    fn write(&self) -> Result<Vec<u8>>;
}
