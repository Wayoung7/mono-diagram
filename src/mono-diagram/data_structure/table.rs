use std::fmt::{Debug, Display};

#[derive(Default, Debug)]
pub struct Table<T>
where
    T: Display + Default + Debug,
{
    pub cells: Vec<Vec<TableCell<T>>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Default, Debug)]
pub struct TableCell<T>
where
    T: Display + Default + Debug,
{
    pub value: T,
}
