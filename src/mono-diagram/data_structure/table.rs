use std::fmt::{Debug, Display};

#[derive(Default)]
pub struct Table<T>
where
    T: Display + Default + Debug,
{
    cells: Vec<Vec<TableCell<T>>>,
}

#[derive(Default)]
pub struct TableCell<T>
where
    T: Display + Default + Debug,
{
    value: T,
}
