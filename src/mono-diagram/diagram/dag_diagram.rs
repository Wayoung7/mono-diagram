use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    cmp::{max, min},
    collections::{HashMap, HashSet},
    iter::repeat,
};

use anyhow::Error;
use pest::Parser;
use pest_derive::Parser;
use petgraph::{
    graph::{DiGraph, EdgeIndex, NodeIndex},
    visit::{Dfs, EdgeRef, IntoEdgeReferences, IntoNeighbors, IntoNeighborsDirected},
    Direction::{Incoming, Outgoing},
    Graph,
};
use rand::{seq::IteratorRandom, thread_rng, Rng};

use super::Diagram;

#[derive(Debug, Default)]
pub struct DagGraph {
    data: DiGraph<NodeData, ()>,
}

impl Diagram for DagGraph {
    fn parse_from_str(&mut self, input: &str) -> anyhow::Result<()> {
        let mut assign_map: HashMap<&str, &str> = HashMap::new();
        let mut relationship_map: HashSet<(&str, &str)> = HashSet::new();
        let diagram = DagGraphParser::parse(Rule::diagram, input)
            .map_err(|e| {
                Error::msg(format!(
                    "parsing error: incorrect binary_tree grammar, context: {}",
                    e.line()
                ))
            })?
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
                    let mut r = line.into_inner();
                    let mut from = r.next().unwrap();
                    while let Some(to) = r.next() {
                        relationship_map.insert((from.as_str(), to.as_str()));
                        from = to;
                    }
                }
                _ => (),
            }
        }
        // println!("{:#?}\n\n{:#?}", relationship_map, assign_map);
        let dag: Graph<NodeData, ()> = init_dag(&relationship_map);
        let (mut dag, level) = assign_level(dag);
        dag = replace_text(add_dummy(dag), &assign_map);
        let levels = level_cnt(&dag, level);
        dag = permute(dag, &levels);
        // print_dag(&dag);
        print_layer(&dag, level);
        Ok(())
    }

    fn write(&self) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

fn init_dag(r: &HashSet<(&str, &str)>) -> DiGraph<NodeData, ()> {
    let mut g = DiGraph::new();
    let mut added: HashMap<&str, usize> = HashMap::new();
    for (s, t) in r.iter() {
        if !added.contains_key(*s) {
            let i = g.add_node(NodeData {
                value: s.to_string(),
                ..Default::default()
            });
            added.insert(*s, i.index());
        }
        if !added.contains_key(*t) {
            let j = g.add_node(NodeData {
                value: t.to_string(),
                ..Default::default()
            });
            added.insert(*t, j.index());
        }
        g.update_edge(NodeIndex::new(added[*s]), NodeIndex::new(added[*t]), ());
    }
    g
}

fn assign_level(g: DiGraph<NodeData, ()>) -> (DiGraph<NodeData, ()>, usize) {
    let mut res = g.clone();
    let mut max_level = 0;
    g.node_indices()
        .filter(|n| g.neighbors_directed(*n, Incoming).next().is_none())
        .for_each(|i| {
            res[i].level = 1;
        });
    // let mut dfs = Dfs::new(&res, NodeIndex::new(0));
    // while let Some(n) = dfs.next(&res) {
    //     if res.neighbors_directed(n, Incoming).next().is_some() {
    //         res[n].level = cal_level(&res, n);
    //     }
    // }
    // print_dag(&res);
    res.node_indices()
        .filter(|n| g.neighbors_directed(*n, Incoming).next().is_some())
        .for_each(|i| {
            let level = cal_level(&res, i);
            if level > max_level {
                max_level = level;
            }
            res[i].level = cal_level(&res, i);
        });
    (res, max_level)
}

fn add_dummy(g: DiGraph<NodeData, ()>) -> DiGraph<NodeData, ()> {
    let mut res = g.clone();
    let mut edge_to_remove: Vec<EdgeIndex> = Vec::new();
    for e in g.edge_indices() {
        let s = g.edge_endpoints(e).unwrap().0;
        let t = g.edge_endpoints(e).unwrap().1;
        if g[s].level.abs_diff(g[t].level) > 1 {
            let start = g[s].level;
            let end = g[t].level;
            edge_to_remove.push(e);
            let mut last_node = s;
            ((start + 1)..end).for_each(|i| {
                let cur_node = res.add_node(NodeData {
                    level: i,
                    dummy: true,
                    ..Default::default()
                });
                res.update_edge(last_node, cur_node, ());
                last_node = cur_node;
            });
            res.update_edge(last_node, t, ());
            // res.remove_edge(e);
        }
    }
    for e in edge_to_remove.into_iter() {
        res.remove_edge(e);
    }
    res
}

fn level_cnt(g: &DiGraph<NodeData, ()>, level: usize) -> Vec<usize> {
    let mut res: Vec<usize> = repeat(0).take(level).collect();
    for n in g.node_indices() {
        res[g[n].level - 1] += 1;
    }
    res
}

fn replace_text(g: DiGraph<NodeData, ()>, a: &HashMap<&str, &str>) -> DiGraph<NodeData, ()> {
    let mut res = g.clone();
    for n in g.node_indices() {
        let mut len = 0;
        if a.contains_key(g[n].value.as_str()) {
            res[n].value = a[g[n].value.as_str()].to_string();
            if res[n].value.len() > len {
                len = res[n].value.len();
            }
        }
        if g.neighbors_directed(n, Incoming).count() > len {
            len = g.neighbors_directed(n, Incoming).count();
        }
        if g.neighbors_directed(n, Outgoing).count() > len {
            len = g.neighbors_directed(n, Outgoing).count();
        }
        res[n].width = if g[n].dummy { 0 } else { len + 2 };
        res[n].height = 3;
    }
    res
}

fn permute(g: DiGraph<NodeData, ()>, levels: &Vec<usize>) -> DiGraph<NodeData, ()> {
    // let mut permutation: HashMap<usize, Vec<NodeIndex>> = HashMap::new();
    // for n in g.node_indices() {
    //     permutation
    //         .entry(g[n].level)
    //         .and_modify(|v| v.push(n))
    //         .or_insert_with(|| vec![n]);
    // }
    let mut res = g.clone();
    for l in 1..=levels.len() {
        let mut perm = 0;
        g.node_indices().filter(|i| g[*i].level == l).for_each(|i| {
            res[i].permutation = perm;
            res[i].temp_perm = perm as f32;
            perm += 1;
        });
    }
    let largest_level = levels
        .iter()
        .enumerate()
        .max_by_key(|&(_, &val)| val)
        .map(|(index, _)| index)
        .unwrap()
        + 1;
    println!("{}", largest_level);
    let mut outer_optimal = res.clone();
    let mut outer_optimal_crossings = cal_crossings(&res, levels.len());
    const OUTER_MAX_LOOP: usize = 23;
    let mut outer_loop_cnt = 0;
    loop {
        if outer_loop_cnt >= OUTER_MAX_LOOP {
            break;
        }
        // res = rand_permute_level(
        //     res.clone(),
        //     largest_level,
        //     levels[largest_level.wrapping_sub(1)],
        // );
        let inner = {
            let mut inner_optimal = get_rand_permute_level(
                res.clone(),
                largest_level,
                levels[largest_level.wrapping_sub(1)],
            );
            // print_layer(&inner_optimal, levels.len());
            let mut inner_optimal_crossings = cal_crossings(&inner_optimal, levels.len());
            const INNER_MAX_LOOP: usize = 23;
            let mut inner_loop_cnt = 0;
            let mut step: isize = -1;
            let mut current_level = largest_level;
            loop {
                if inner_loop_cnt >= INNER_MAX_LOOP {
                    break;
                }

                current_level = current_level.wrapping_add_signed(step);
                if current_level <= 1 {
                    step = 1;
                    inner_loop_cnt += 1;
                }
                if current_level >= levels.len() {
                    step = -1;
                    inner_loop_cnt += 1;
                }
                let mut new_res = res.clone();
                let mut perms: Vec<(NodeIndex, f32)> = Vec::new();
                for n in res.node_indices() {
                    if res[n].level == current_level.wrapping_add_signed(step) {
                        if res
                            .neighbors_directed(n, if step == -1 { Outgoing } else { Incoming })
                            .next()
                            .is_some()
                        {
                            let temp_perm = res
                                .neighbors_directed(n, if step == -1 { Outgoing } else { Incoming })
                                .fold(0, |acc, c| acc + res[c].permutation)
                                as f32
                                / res
                                    .neighbors_directed(
                                        n,
                                        if step == -1 { Outgoing } else { Incoming },
                                    )
                                    .count() as f32;
                            new_res[n].temp_perm = temp_perm;
                            perms.push((n, temp_perm));
                        } else {
                            perms.push((n, res[n].temp_perm));
                        }
                    }
                }
                perms.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                for (idx, (i, _)) in perms.into_iter().enumerate() {
                    new_res[i].permutation = idx;
                }
                res = new_res;
                let res_crossings = cal_crossings(&res, levels.len());
                if res_crossings < inner_optimal_crossings {
                    inner_optimal_crossings = res_crossings;
                    inner_optimal = res.clone();
                }
            }
            (inner_optimal, inner_optimal_crossings)
        };
        print_layer(&inner.0, levels.len());
        if inner.1 < outer_optimal_crossings {
            outer_optimal_crossings = inner.1;
            outer_optimal = inner.0;
        }
        // println!("Crossing: {}", outer_optimal_crossings);
        outer_loop_cnt += 1;
    }
    outer_optimal
}

fn get_rand_permute_level(
    g: DiGraph<NodeData, ()>,
    level: usize,
    level_len: usize,
) -> DiGraph<NodeData, ()> {
    let mut res = g.clone();
    let sequence: Vec<usize> = (0..level_len).collect();
    let mut rand_perm = sequence.clone();
    for i in 1..level_len {
        let j = thread_rng().gen_range(0..=i);
        rand_perm[i] = rand_perm[j];
        rand_perm[j] = sequence[i];
    }
    let mut cur = 0;
    for n in g.node_indices() {
        if g[n].level == level {
            res[n].permutation = rand_perm[cur];
            cur += 1;
        }
    }
    res
}

fn print_layer(g: &DiGraph<NodeData, ()>, level: usize) {
    println!("");
    for l in 1..=level {
        let mut line: Vec<(usize, &str)> = Vec::new();
        for n in g.node_indices() {
            if g[n].level == l {
                line.push((
                    g[n].permutation,
                    if g[n].dummy { "#" } else { g[n].value.as_str() },
                ));
            }
        }
        line.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for p in line.iter() {
            print!("{} ", p.1);
        }
        println!("");
    }
    println!("");
    // for l in 1..=level {
    //     let mut perm = 0;
    //     for n in g.node_indices() {
    //         if g[n].level == l && g[n].permutation == perm {
    //             print!("{:?} ", g[n].temp_perm);
    //             perm += 1;
    //         }
    //     }
    //     println!("");
    // }
    println!("Crossings: {}", cal_crossings(g, level));
    print!("\n");
}

fn cal_crossings(g: &DiGraph<NodeData, ()>, level: usize) -> usize {
    let mut cnt = 0;
    for l in 1..level {
        let mut lines_level: Vec<(usize, usize)> = Vec::new();
        for e in g.edge_indices() {
            if g[g.edge_endpoints(e).unwrap().0].level == l {
                lines_level.push((
                    g[g.edge_endpoints(e).unwrap().0].permutation,
                    g[g.edge_endpoints(e).unwrap().1].permutation,
                ));
            }
        }
        cnt += crossings(&lines_level);
    }
    cnt
}

fn print_dag(g: &DiGraph<NodeData, ()>) {
    // let mut dfs = Dfs::new(g, NodeIndex::new(0));
    for n in g.node_indices() {
        println!(
            "Index: {}, Level: {}, Value: {}, Dummy: {:?}, Size: ({}, {}), Permutation: {}, Incoming: {}, OutGoing: {}",
            n.index(),
            g[n].level,
            g[n].value,
            g[n].dummy,
            g[n].width,
            g[n].height,
            g[n].permutation,
            g.neighbors_directed(n, Incoming)
                .map(|neighbor| &g[neighbor].value)
                .flat_map(|s| s.chars())
                .collect::<String>(),
            g.neighbors_directed(n, Outgoing)
                .map(|neighbor| &g[neighbor].value)
                .flat_map(|s| s.chars())
                .collect::<String>(),
        );
    }
    // for n in g.edge_indices() {
    //     println!(
    //         "Edge: ({}, {}) ",
    //         g[g.edge_endpoints(n).unwrap().0].value,
    //         g[g.edge_endpoints(n).unwrap().1].value
    //     );
    // }
}

fn cal_level(g: &DiGraph<NodeData, ()>, i: NodeIndex) -> usize {
    if g[i].level == 1 {
        1
    } else {
        g.neighbors_directed(i, Incoming)
            .map(|n| {
                if g[n].level == 0 {
                    cal_level(g, n)
                } else {
                    g[n].level
                }
            })
            .max()
            .unwrap()
            + 1
    }
}

fn crossings(lines: &Vec<(usize, usize)>) -> usize {
    if lines.len() <= 1 {
        0
    } else {
        let mut left = 0;
        let mut right = 1;
        let mut cnt = 0;
        while right < lines.len() {
            while left < right {
                if has_crossing(lines[left], lines[right]) {
                    cnt += 1;
                }
                left += 1;
            }

            right += 1;
            left = 0;
        }
        cnt
    }
}

fn has_crossing(a: (usize, usize), b: (usize, usize)) -> bool {
    (a.0 as isize - b.0 as isize) * (a.1 as isize - b.1 as isize) < 0
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NodeData {
    pub level: usize,
    pub width: usize,
    pub height: usize,
    pub pos: (usize, usize),
    pub value: String,
    pub dummy: bool,
    pub permutation: usize,
    pub temp_perm: f32,
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/dag.pest"]
pub struct DagGraphParser;
