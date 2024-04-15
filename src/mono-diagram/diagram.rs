pub mod binary_tree_diagram;
pub mod grid;
pub mod table_diagram;

pub trait Diagram {
    fn parse_from_str(&mut self, input: &str);
    fn print(&self);
}
