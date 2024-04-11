use std::fmt::{Debug, Display};

pub struct TreeNode<T: Display + Default> {
    value: T,
    lnode: Option<Box<TreeNode<T>>>,
    rnode: Option<Box<TreeNode<T>>>,
}

impl<T: Display + Default> TreeNode<T> {
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

    pub fn map<F>(&mut self, f: &F)
    where
        F: Fn(&mut T),
    {
        f(&mut self.value);
        if let Some(ref mut left) = self.lnode {
            left.map(f);
        }
        if let Some(ref mut right) = self.rnode {
            right.map(f);
        }
    }
}

impl<T: Display + Default> Display for TreeNode<T> {
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

impl<T: Display + Default> Default for TreeNode<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            lnode: None,
            rnode: None,
        }
    }
}
