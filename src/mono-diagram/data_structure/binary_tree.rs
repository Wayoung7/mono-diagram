use std::{
    cmp::max,
    fmt::{Debug, Display},
};

pub struct TreeNode<T>
where
    T: Display + Default + Debug,
{
    pub value: T,
    pub lnode: Option<Box<TreeNode<T>>>,
    pub rnode: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Display + Default + Debug,
{
    pub fn new(value: T, lnode: Option<Box<TreeNode<T>>>, rnode: Option<Box<TreeNode<T>>>) -> Self {
        Self {
            value,
            lnode,
            rnode,
        }
    }

    pub fn new_leaf(value: T) -> Self {
        Self {
            value,
            lnode: None,
            rnode: None,
        }
    }

    pub fn degree(&self) -> usize {
        max(
            if let Some(l) = &self.lnode {
                l.degree() + 1
            } else {
                1
            },
            if let Some(r) = &self.rnode {
                r.degree() + 1
            } else {
                1
            },
        )
    }
}

impl<T> Display for TreeNode<T>
where
    T: Display + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        write!(f, "Root: {}", self.value)?;
        if let Some(ref left) = self.lnode {
            write!(f, "  Left: {}", left)?;
        }
        if let Some(ref right) = self.rnode {
            write!(f, "  Right: {}", right)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T> Default for TreeNode<T>
where
    T: Display + Default + Debug,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            lnode: None,
            rnode: None,
        }
    }
}

impl<T> Debug for TreeNode<T>
where
    T: Display + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self.value)?;
        Ok(())
    }
}
