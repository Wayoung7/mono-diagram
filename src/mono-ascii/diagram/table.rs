use super::Diagram;

#[derive(Default)]
pub struct TableDiagram {}

impl Diagram for TableDiagram {
    fn parse_from_str(&mut self, input: &str) {
        println!("{input}");
    }

    fn print(&self) {}
}
