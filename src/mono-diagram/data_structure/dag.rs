use std::{
    cell::{Cell, RefCell},
    fmt::{Debug, Display},
    rc::Rc,
};

use typed_arena::Arena;

#[derive(Default)]
pub struct DAG<'a, T>
where
    T: Debug + Default + Display,
{
    // pub vertices: Vec<GraphVertex<T>>,
    // pub edges: Vec<GraphEdge<T>>,
    pub vertices: Arena<GraphVertex<'a, T>>,
}

impl<'a, T> Debug for DAG<'a, T>
where
    T: Debug + Default + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.vertices.len())
    }
}

#[derive(Default)]
pub struct GraphVertex<'a, T>
where
    T: Debug + Default + Display,
{
    // pub parents: Vec<Box<GraphNode<T>>>,
    // pub childs: Vec<Rc<RefCell<GraphVertex<T>>>>,
    pub value: T,
    pub childs: Vec<Cell<&'a GraphVertex<'a, T>>>,
}

// pub struct GraphEdge<T>
// where
//     T: Debug + Default + Display,
// {
//     pub source: Box<GraphVertex<T>>,
//     pub target: Box<GraphVertex<T>>,
// }

impl<'a, T> GraphVertex<'a, T>
where
    T: Debug + Default + Display,
{
    pub fn new(value: T) -> Self
    where
        T: Debug + Default + Display,
    {
        Self {
            childs: Vec::new(),
            value,
        }
    }
}

impl<'a, T> DAG<'a, T>
where
    T: Debug + Default + Display + Eq,
{
    pub fn add_vertex(&self, vertex: GraphVertex<'a, T>) -> &mut GraphVertex<'a, T> {
        self.vertices.alloc(vertex)
    }

    pub fn add_edge(source: &mut GraphVertex<'a, T>, target: &'a GraphVertex<'a, T>) {
        source.childs.push(Cell::new(target));
    }

    pub fn get_vertex(&mut self, value: T) -> &mut GraphVertex<'a, T> {
        for v in self.vertices.iter_mut() {
            if v.value == value {
                return v;
            }
        }
        self.add_vertex(GraphVertex::new(value))
    }
}

pub fn test() {
    let dag: DAG<'_, String> = DAG::default();
    println!("{dag:?}");
    let a = dag.add_vertex(GraphVertex {
        value: "a".to_string(),
        childs: Vec::new(),
    });

    let b = dag.add_vertex(GraphVertex {
        value: "b".to_string(),
        childs: Vec::new(),
    });

    DAG::add_edge(a, b);
    println!("{dag:?}");
}
