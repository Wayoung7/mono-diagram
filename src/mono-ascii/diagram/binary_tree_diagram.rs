use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::data_structure::binary_tree::TreeNode;

use super::Diagram;

pub struct BinaryTreeDiagram {
    data: Box<TreeNode<String>>,
}

impl Default for BinaryTreeDiagram {
    fn default() -> Self {
        Self {
            data: Box::new(TreeNode::default()),
        }
    }
}

impl Diagram for BinaryTreeDiagram {
    fn parse_from_str(&mut self, input: &str) {
        let mut root: &str = "";
        let mut relationship_map: HashMap<&str, (Option<&str>, Option<&str>)> = HashMap::new();
        let mut assign_map: HashMap<&str, &str> = HashMap::new();
        let diagram = BinaryTreeParser::parse(Rule::diagram, input)
            .unwrap()
            .next()
            .unwrap();
        for line in diagram.into_inner() {
            match line.as_rule() {
                Rule::assign => {
                    let mut statement = line.into_inner();
                    let variable = statement.next().unwrap().as_str();
                    let value = statement.next().unwrap().as_str();
                    assign_map.insert(variable, value);
                }
                Rule::relationship => {
                    let mut statement = line.into_inner();
                    let node_variable_name = statement.next().unwrap().as_str();
                    let childs = parse_childs(statement.next().unwrap().into_inner());
                    relationship_map.insert(node_variable_name, childs);
                    if root == "" {
                        root = node_variable_name;
                    }
                    if let Some(lchild) = childs.0 {
                        if root == lchild {
                            root = node_variable_name;
                        }
                    }
                    if let Some(rchild) = childs.1 {
                        if root == rchild {
                            root = node_variable_name;
                        }
                    }
                }
                _ => (),
            }
        }

        let tree = construct_tree(root, &relationship_map, &assign_map);
        // println!("{}", tree);
        *self = BinaryTreeDiagram { data: tree };
    }

    fn print(&self) {}
}

fn parse_childs(pairs: Pairs<Rule>) -> (Option<&str>, Option<&str>) {
    let mut res = (None, None);
    for pair in pairs {
        match pair.as_rule() {
            Rule::lchild => res.0 = Some(pair.as_str()),
            Rule::rchild => res.1 = Some(pair.as_str()),
            _ => (),
        }
    }

    res
}

fn construct_tree<'a>(
    root: &'a str,
    rm: &'a HashMap<&'a str, (Option<&'a str>, Option<&'a str>)>,
    am: &'a HashMap<&'a str, &'a str>,
) -> Box<TreeNode<String>> {
    construct_tree_helper(root, rm, am, &RefCell::new(HashSet::new()))
}

fn construct_tree_helper<'a>(
    root: &'a str,
    rm: &'a HashMap<&'a str, (Option<&'a str>, Option<&'a str>)>,
    am: &'a HashMap<&'a str, &'a str>,
    set: &RefCell<HashSet<String>>,
) -> Box<TreeNode<String>> {
    set.borrow_mut().insert(root.to_string());

    if let Some(childs) = rm.get(&root) {
        let mut lchild = None;
        let mut rchild = None;
        if let Some(lc) = childs.0 {
            if !set.borrow().contains(lc) {
                lchild = Some(construct_tree_helper(lc, rm, am, set));
            }
        }
        if let Some(rc) = childs.1 {
            if !set.borrow().contains(rc) {
                rchild = Some(construct_tree_helper(rc, rm, am, set));
            }
        }
        Box::new(TreeNode::new(
            if let Some(value) = am.get(&root) {
                (*value).to_string()
            } else {
                root.to_string()
            },
            lchild,
            rchild,
        ))
    } else {
        Box::new(TreeNode::new_leaf(if let Some(value) = am.get(&root) {
            (*value).to_string()
        } else {
            root.to_string()
        }))
    }
}

#[derive(Parser)]
#[grammar = "mono-ascii/grammar/binary_tree.pest"]
struct BinaryTreeParser;
