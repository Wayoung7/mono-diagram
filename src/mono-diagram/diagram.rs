pub mod binary_tree_diagram;
pub mod grid_diagram;
pub mod table_diagram;

pub trait Diagram {
    fn parse_from_str(&mut self, input: &str);
    fn print(&self);
}

// +-----------+------------+---------------+-------------+
// | 基类成员  | Public继承 | Protected继承 | Private继承 |
// +-----------+------------+---------------+-------------+
// | Public    | Public     | Protected     | Private     |
// +-----------+------------+---------------+-------------+
// | Protected | Protected  | Protected     | Private     |
// +-----------+------------+---------------+-------------+
// | Private   | Hidden     | Hidden        | Hidden      |
// +-----------+------------+---------------+-------------+
