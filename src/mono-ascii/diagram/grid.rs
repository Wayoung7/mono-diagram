use super::Diagram;

#[derive(Default)]
pub struct GridDiagram {}

impl Diagram for GridDiagram {
    fn parse_from_str(&mut self, input: &str) {
        println!("{input}");
    }

    fn print(&self) {}
}
