use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use anyhow::Error;
use pest::Parser;
use pest_derive::Parser;
use petgraph::{
    algo::is_cyclic_directed,
    graph::{DiGraph, EdgeIndex, NodeIndex},
    visit::EdgeRef,
    Direction::{self, Incoming, Outgoing},
};
use rand::{thread_rng, Rng};

use super::Diagram;

const PALETTE: &str = "┌─┐││└─┘┬│─└┐┌┘V";
type Digraph = DiGraph<NodeData, EdgeData>;

#[derive(Debug, Default)]
pub struct DagGraph {
    data: Digraph,
    max_width: usize,
    max_height: usize,
    connections: Vec<Vec<Connection>>,
    spacing: Vec<(isize, isize)>,
    line_height: Vec<usize>,
}

impl Diagram for DagGraph {
    fn parse_from_str(&mut self, input: &str) -> anyhow::Result<()> {
        let mut assign_map: HashMap<&str, &str> = HashMap::new();
        let mut relationship_map: HashSet<(&str, &str)> = HashSet::new();
        let diagram = DagGraphParser::parse(Rule::diagram, input)
            .map_err(|e| {
                Error::msg(format!(
                    "parsing error: incorrect dag grammar, context: {}",
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
                    for to in r {
                        relationship_map.insert((from.as_str(), to.as_str()));
                        from = to;
                    }
                }
                _ => (),
            }
        }
        // println!("{:#?}\n\n{:#?}", relationship_map, assign_map);
        let dag = init_dag(&relationship_map);
        if is_cyclic_directed(&dag) {
            return Err(Error::msg(
                "diagram error: directed acyclic graph has cycles",
            ));
        }
        let (mut dag, max_level) = assign_level(dag);
        dag = replace_text(add_dummy(dag), &assign_map);
        let levels = level_cnt(&dag, max_level);
        dag = permute(dag, &levels);
        let perm_levels = get_perm_levels(&dag, max_level);
        let (_w, _, line_height) = place_node(&mut dag, &levels, &perm_levels);
        // for i in dag.edge_indices() {
        //     let (s, t) = dag.edge_endpoints(i).unwrap();
        //     println!("{}->{}", dag[s].value, dag[t].value);
        // }
        // print!("\n");
        let width_shift;
        (self.connections, self.spacing, width_shift) = add_connections(&mut dag, &perm_levels);
        self.max_width = _w + width_shift;
        self.max_height = self
            .spacing
            .iter()
            .map(|s| (s.1 - s.0) as usize)
            .sum::<usize>()
            + line_height.iter().sum::<usize>();
        self.line_height = line_height;
        // println!("{}, {}", _w, self.max_height);
        // println!("{:#?}", self.connections);
        // print_dag(&dag);

        // print_layer(&dag, max_level);
        self.data = dag;
        Ok(())
    }

    fn write(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![vec![' '; self.max_width]; self.max_height];
        // println!("buffer: {}, {}", buffer[0].len(), buffer.len());
        for n in self.data.node_indices() {
            let (x, y) = self.data[n].pos;
            let (w, h) = (self.data[n].width, self.data[n].height);
            // println!("{} {} {} {}", x, y, w, h);

            // Draw corner
            if self.data[n].dummy {
                buffer[y][x] = PALETTE.chars().nth(9).unwrap();
                buffer[y + h - 1][x] = PALETTE.chars().nth(9).unwrap();
            } else {
                buffer[y][x] = PALETTE.chars().nth(0).unwrap();
                buffer[y][x + w - 1] = PALETTE.chars().nth(2).unwrap();
                buffer[y + h - 1][x] = PALETTE.chars().nth(5).unwrap();
                buffer[y + h - 1][x + w - 1] = PALETTE.chars().nth(7).unwrap();
            }
            // Draw edge
            for _x in (x + 1)..(x + w - 1) {
                buffer[y][_x] = PALETTE.chars().nth(1).unwrap();
                buffer[y + h - 1][_x] = PALETTE.chars().nth(6).unwrap();
            }
            for line in buffer.iter_mut().take(y + h - 1).skip(y + 1) {
                line[x] = PALETTE.chars().nth(3).unwrap();
                line[x + w - 1] = PALETTE.chars().nth(4).unwrap();
            }
            for (idx, c) in self.data[n].value.chars().enumerate() {
                buffer[y + 1][x + 2 + idx] = c;
            }
        }

        // Draw straight connections
        for l in self.connections.iter() {
            for con in l {
                if let Connection::Straight { from, dummy } = con {
                    let mut _y = from.1 + 1;
                    while buffer[_y][from.0] == ' ' {
                        buffer[_y][from.0] = PALETTE.chars().nth(9).unwrap();
                        _y += 1;
                    }
                    if !dummy.0 {
                        buffer[from.1][from.0] = PALETTE.chars().nth(8).unwrap();
                    }
                    if !dummy.1 {
                        buffer[_y][from.0] = PALETTE.chars().nth(15).unwrap();
                    }
                }
            }
        }

        // Draw other connections
        for (idx, l) in self.connections.iter().enumerate() {
            for con in l {
                if let Connection::Bent { x1, x2, y, dummy } = con {
                    let y0 = self.line_height.iter().take(idx + 1).sum::<usize>()
                        + self
                            .spacing
                            .iter()
                            .map(|s| (s.1 - s.0) as usize)
                            .take(idx)
                            .sum::<usize>();
                    let _y = y0 + (y - self.spacing[idx].0) as usize;
                    // Draw corner
                    if x1 < x2 {
                        buffer[_y][*x1] = PALETTE.chars().nth(11).unwrap();
                        buffer[_y][*x2] = PALETTE.chars().nth(12).unwrap();
                    } else {
                        buffer[_y][*x1] = PALETTE.chars().nth(14).unwrap();
                        buffer[_y][*x2] = PALETTE.chars().nth(13).unwrap();
                    }
                    // Draw first vertical
                    let mut __y = _y - 1;
                    while __y < self.max_height && *x1 < self.max_width && buffer[__y][*x1] == ' ' {
                        buffer[__y][*x1] = PALETTE.chars().nth(9).unwrap();
                        __y -= 1;
                    }
                    if !dummy.0 {
                        buffer[__y][*x1] = PALETTE.chars().nth(8).unwrap();
                    }

                    // Draw second vertical
                    __y = _y + 1;
                    while __y < self.max_height && *x2 < self.max_width && buffer[__y][*x2] == ' ' {
                        buffer[__y][*x2] = PALETTE.chars().nth(9).unwrap();
                        __y += 1;
                    }
                    if !dummy.1 {
                        buffer[__y][*x2] = PALETTE.chars().nth(15).unwrap();
                    }

                    // Draw horizontal
                    // let (_x1, _x2) = (min(x1, x2), max(x1, x2));
                    // for h in *_x1 + 1..*_x2 {
                    //     if buffer[_y][h] == ' ' {
                    //         buffer[_y][h] = PALETTE.chars().nth(10).unwrap();
                    //     }
                    // }
                }
            }
        }
        for (idx, l) in self.connections.iter().enumerate() {
            for con in l {
                if let Connection::Bent { x1, x2, y, .. } = con {
                    let y0 = self.line_height.iter().take(idx + 1).sum::<usize>()
                        + self
                            .spacing
                            .iter()
                            .map(|s| (s.1 - s.0) as usize)
                            .take(idx)
                            .sum::<usize>();
                    let _y = y0 + (y - self.spacing[idx].0) as usize;
                    // println!("y: {}, x1: {}, x2: {}", _y, x1, x2);
                    // Draw horizontal
                    let (_x1, _x2) = (min(x1, x2), max(x1, x2));
                    for h in *_x1 + 1..*_x2 {
                        if _y < buffer.len() && h < buffer[0].len() && buffer[_y][h] == ' ' {
                            buffer[_y][h] = PALETTE.chars().nth(10).unwrap();
                        }
                    }
                }
            }
        }

        let mut res: Vec<u8> = Vec::new();
        for row in buffer.iter() {
            for c in row.iter() {
                res.extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes());
            }
            res.push(b'\n');
        }
        Ok(res)
    }
}

fn get_perm_levels(g: &Digraph, max_level: usize) -> Vec<Vec<NodeIndex>> {
    let mut res = vec![vec![]; max_level];
    for n in g.node_indices() {
        res[g[n].level - 1].push((n, g[n].permutation));
    }
    res.into_iter()
        .map(|mut r| {
            r.sort_by(|a, b| a.1.cmp(&b.1));
            r.into_iter().map(|(a, _)| a).collect()
        })
        .collect()
}

fn place_node(
    g: &mut Digraph,
    levels: &[usize],
    perm_levels: &Vec<Vec<NodeIndex>>,
) -> (usize, usize, Vec<usize>) {
    fn shift_right(
        g: &mut Digraph,
        perm_levels: &[Vec<NodeIndex>],
        level: usize,
        perm: usize,
        amount: usize,
    ) {
        if perm >= perm_levels[level - 1].len() {
            return;
        }
        for n in perm_levels[level - 1][perm..].iter() {
            g[*n].pos.0 += amount;
        }
    }

    let mut cur_row = 0;
    let mut max_col = 0;

    for r in perm_levels {
        let mut cur_col = 0;
        for c in r {
            g[*c].pos = (cur_col, cur_row);
            cur_col += g[*c].width + 1;
        }
        max_col = max(cur_col, max_col);
        cur_row += 3;
    }
    let row_height: Vec<usize> = vec![3; levels.len()];

    let cl = cal_crossings_levels(g, levels.len());
    let mut max_shift_right = 0;

    const MAX_LOOP: usize = 3;
    let mut loop_cnt = 0;
    while loop_cnt < MAX_LOOP {
        for (idx1, two_r) in perm_levels.windows(2).enumerate() {
            // println!("{}.", idx1);
            // print_layer(g, perm_levels);
            if cl[idx1] != 0 {
                continue;
            }
            for (idx2, n) in two_r[1].iter().enumerate() {
                let new_x = g.neighbors_directed(*n, Incoming).fold(0, |acc, x| {
                    let new_x = if g[*n].dummy && g[x].dummy {
                        g[x].pos.0
                    } else if !g[*n].dummy && !g[x].dummy {
                        g[x].pos.0 + 2
                    } else {
                        g[x].pos.0 + 1
                    };
                    if new_x > acc {
                        new_x
                    } else {
                        acc
                    }
                });
                let shift: isize = new_x as isize - g[*n].pos.0 as isize - g[*n].width as isize + 1;
                if shift > 0 {
                    if g[*n].dummy {
                        g[*n].pos.0 += shift as usize;
                    } else {
                        g[*n].width += shift as usize;
                    }
                    max_shift_right = max(max_shift_right, shift as usize);
                    shift_right(g, perm_levels, idx1 + 2, idx2 + 1, shift as usize);
                }
            }
        }

        for (idx1, two_r) in perm_levels.windows(2).enumerate().rev() {
            // println!("{}.", idx1);
            // print_layer(g, perm_levels);
            if cl[idx1] != 0 {
                continue;
            }
            for (idx2, n) in two_r[0].iter().enumerate() {
                let new_x = g.neighbors_directed(*n, Outgoing).fold(0, |acc, x| {
                    let new_x = if g[*n].dummy && g[x].dummy {
                        g[x].pos.0
                    } else if !g[*n].dummy && !g[x].dummy {
                        g[x].pos.0 + 2
                    } else {
                        g[x].pos.0 + 1
                    };
                    if new_x > acc {
                        new_x
                    } else {
                        acc
                    }
                });
                let shift: isize = new_x as isize - g[*n].pos.0 as isize - g[*n].width as isize + 1;
                if shift > 0 {
                    if g[*n].dummy {
                        g[*n].pos.0 += shift as usize;
                    } else {
                        g[*n].width += shift as usize;
                    }
                    max_shift_right = max(max_shift_right, shift as usize);
                    shift_right(g, perm_levels, idx1 + 1, idx2 + 1, shift as usize);
                }
            }
        }
        loop_cnt += 1;
    }

    (max_col + max_shift_right, cur_row, row_height)
}

fn add_connections(
    g: &mut Digraph,
    perm_levels: &[Vec<NodeIndex>],
) -> (Vec<Vec<Connection>>, Vec<(isize, isize)>, usize) {
    fn shift_down(g: &mut Digraph, perm_levels: &[Vec<NodeIndex>], level: usize, amount: usize) {
        for r in &perm_levels[level - 1..] {
            for c in r {
                g[*c].pos.1 += amount;
            }
        }
    }
    fn shift_right_one(g: &mut Digraph, connections: &mut [Vec<Connection>], x: usize) {
        for n in g.node_weights_mut() {
            if n.pos.0 >= x {
                n.pos.0 += 1;
            } else if n.pos.0 < x && n.pos.0 + n.width > x {
                n.width += 1;
            }
        }
        for r in connections.iter_mut() {
            for con in r.iter_mut() {
                match con {
                    Connection::Straight { from, .. } => {
                        if from.0 >= x {
                            from.0 += 1;
                        }
                    }
                    Connection::Bent { x1, x2, .. } => {
                        if *x1 >= x {
                            *x1 += 1;
                        }
                        if *x2 >= x {
                            *x2 += 1;
                        }
                    }
                }
            }
        }
    }
    fn overlap(
        (x1, w1, dummy1): (usize, usize, bool),
        (x2, w2, dummy2): (usize, usize, bool),
    ) -> Option<usize> {
        if dummy1 && dummy2 {
            if x1 == x2 {
                Some(x1)
            } else {
                None
            }
        } else if dummy1 {
            if x1 > x2 && x1 < x2 + w2 - 1 {
                Some(x1)
            } else {
                None
            }
        } else if dummy2 {
            if x2 > x1 && x2 < x1 + w1 - 1 {
                Some(x2)
            } else {
                None
            }
        } else if x1 + w1 - 1 < x2 + 2 || x2 + w2 - 1 < x1 + 2 {
            None
        } else {
            Some(max(x1, x2) + 1)
        }
    }

    let mut ct: Vec<Vec<Connection>> = vec![vec![]; perm_levels.len() - 1];

    fn connections_contain_x(c: &[Vec<Connection>], i: usize, x: usize, d: Direction) -> bool {
        for con in &c[i] {
            if let Connection::Straight { from, .. } = con {
                if from.0 == x {
                    return true;
                }
            }
            if let Connection::Bent { x1, x2, .. } = con {
                if d == Outgoing && *x1 == x {
                    return true;
                }
                if d == Incoming && *x2 == x {
                    return true;
                }
            }
        }
        false
    }

    fn bent_overlap(
        c: &[Vec<Connection>],
        i: usize,
        y_spacing: isize,
        x3: usize,
        x4: usize,
    ) -> bool {
        for con in &c[i] {
            if let Connection::Bent { x1, x2, y, .. } = con {
                let (_x1, _x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let (_x3, _x4) = if x3 < x4 { (x3, x4) } else { (x4, x3) };
                if y_spacing == *y {
                    if *_x2 < _x3 || _x4 < *_x1 {
                    } else {
                        return true;
                    }
                }
                if *_x1 == x4 && y_spacing <= *y || *_x2 == x3 && y_spacing >= *y {
                    return true;
                }
            }
        }
        // print!("  <({}, ({:?}, ..)) NOT OVERLAP>  ", y_spacing, (x3, x4));
        false
    }

    fn vertical_overlap(
        c: &[Vec<Connection>],
        i: usize,
        y_spacing: isize,
        x3: usize,
        x4: usize,
    ) -> (bool, bool) {
        let mut res = (false, false);
        for con in &c[i] {
            if let Connection::Bent { x1, x2, y, .. } = con {
                let (_x1, _x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let (_x3, _x4) = if x3 < x4 { (x3, x4) } else { (x4, x3) };
                if *_x1 == x4 && y_spacing < *y {
                    res.1 = true;
                }
                if *_x2 == x3 && y_spacing > *y {
                    res.0 = true;
                }
            }
        }
        res
    }

    // Make straight connections
    for e in g.edge_indices() {
        let (s, t) = g.edge_endpoints(e).unwrap();
        if let Some(x) = overlap(
            (g[s].pos.0, g[s].width, g[s].dummy),
            (g[t].pos.0, g[t].width, g[t].dummy),
        ) {
            ct[g[s].level - 1].push(Connection::Straight {
                from: (x, g[s].pos.1 + g[s].height - 1),
                dummy: (g[s].dummy, g[t].dummy),
            });
            g[e].visited = true;
        }
    }

    // Make other connections
    // for e in g.edge_indices() {
    //     if g[e].visited == false {
    //         let (s, t) = g.edge_endpoints(e).unwrap();
    //     }
    // }

    let mut spacing: Vec<(isize, isize)> = vec![(0, 0); perm_levels.len() - 1];
    let mut shift_right = 0;
    // println!("{:#?}", connections);
    // let connections_bent: Vec<Vec<Connection>> = vec![vec![]; perm_levels.len() - 1];
    // println!(
    //     "{:?}",
    //     connections.iter().map(|l| l.len()).collect::<Vec<_>>()
    // );
    let mut res = g.clone();
    for (idx, r) in perm_levels[..perm_levels.len() - 1].iter().enumerate() {
        for &c in r {
            // For each node
            let mut out_offset = 1;
            let mut childs = g
                .edges_directed(c, Outgoing)
                .filter(|f| !f.weight().visited)
                .collect::<Vec<_>>();
            childs.sort_by(|a, b| g[a.target()].pos.0.cmp(&g[b.target()].pos.0));
            if spacing[idx] == (0, 0) && !childs.is_empty() {
                spacing[idx] = (0, 1);
            }
            for child_edge in childs.into_iter() {
                // For each unvisited child
                assert!(!child_edge.weight().visited);
                let t = child_edge.target();
                out_offset = if res[c].dummy {
                    0
                } else {
                    while connections_contain_x(&ct, idx, res[c].pos.0 + out_offset, Outgoing) {
                        out_offset += 1;
                    }
                    out_offset
                };
                assert!(out_offset <= res[c].width);
                let in_offset = if res[t].dummy {
                    0
                } else {
                    let mut _in_offset = 1;
                    while connections_contain_x(&ct, idx, res[t].pos.0 + _in_offset, Incoming) {
                        _in_offset += 1;
                    }
                    _in_offset
                };
                assert!(in_offset < res[t].width);

                let mut found = false;
                let mut x_from = res[c].pos.0 + out_offset;
                let mut x_to = res[t].pos.0 + in_offset;
                assert!(x_from != x_to);
                for s in if x_to > x_from {
                    spacing[idx].0..spacing[idx].1
                } else {
                    spacing[idx].1..spacing[idx].0
                } {
                    if !bent_overlap(&ct, idx, s, x_from, x_to) {
                        ct[idx].push(Connection::Bent {
                            x1: x_from,
                            x2: x_to,
                            y: s,
                            dummy: (res[c].dummy, res[t].dummy),
                        });
                        found = true;
                        // print!("{}->{}, y: {} ", g[c].value, g[t].value, s);
                        break;
                    }
                }
                if !found {
                    // Space up vertical
                    let y;
                    if x_to > x_from {
                        y = spacing[idx].0 - 1;
                        spacing[idx].0 -= 1;
                    } else {
                        y = spacing[idx].1;
                        spacing[idx].1 += 1;
                    };
                    // Check vertical overlap
                    let vert_overlap_res = vertical_overlap(&ct, idx, y, x_from, x_to);
                    if vert_overlap_res.0 {
                        shift_right_one(&mut res, &mut ct, x_from + 1);
                        x_from += 1;
                        shift_right += 1;
                    }

                    if vert_overlap_res.1 {
                        shift_right_one(&mut res, &mut ct, x_to + 1);
                        x_to += 1;
                        shift_right += 1;
                    }
                    ct[idx].push(Connection::Bent {
                        x1: x_from,
                        x2: x_to,
                        y,
                        dummy: (res[c].dummy, res[t].dummy),
                    });
                }
                // println!(
                //     "{:?}",
                //     connections.iter().map(|l| l.len()).collect::<Vec<_>>()
                // );
                // print!("\n");
                // }
            }
        }
        // println!("{:#?}", connections);
    }
    // println!("{:?}", spacing);
    // Shift things down
    for (idx, shift) in spacing.iter().enumerate() {
        shift_down(&mut res, perm_levels, idx + 2, (shift.1 - shift.0) as usize);
        for r in ct[idx + 1..].iter_mut() {
            for con in r.iter_mut() {
                if let Connection::Straight { from, .. } = con {
                    from.1 += (shift.1 - shift.0) as usize;
                }
            }
        }
    }
    *g = res;
    // g.map(
    //     |_, _| {},
    //     |i, d| {
    //         let (source, target) = g.edge_endpoints(i).unwrap();
    //         if overlap(
    //             (g[source].pos.0, g[source].width),
    //             (g[target].pos.0, g[target].width),
    //         ) {}
    //     },
    // );
    // for r in perm_levels {
    //     for c in r {
    //         for e in g.edges_directed(*c, Outgoing) {
    //             g[e.source()].height = 4;
    //         }
    //     }
    // }

    (ct, spacing, shift_right)
}

fn init_dag(r: &HashSet<(&str, &str)>) -> Digraph {
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
        g.update_edge(
            NodeIndex::new(added[*s]),
            NodeIndex::new(added[*t]),
            EdgeData::default(),
        );
    }
    g
}

fn assign_level(g: Digraph) -> (Digraph, usize) {
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

fn add_dummy(g: Digraph) -> Digraph {
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
                    width: 1,
                    ..Default::default()
                });
                res.update_edge(last_node, cur_node, EdgeData::default());
                last_node = cur_node;
            });
            res.update_edge(last_node, t, EdgeData::default());
            // res.remove_edge(e);
        }
    }
    for e in edge_to_remove.into_iter() {
        res.remove_edge(e);
    }
    res
}

fn level_cnt(g: &Digraph, max_level: usize) -> Vec<usize> {
    let mut res: Vec<usize> = vec![0; max_level];
    for n in g.node_indices() {
        res[g[n].level - 1] += 1;
    }
    res
}

fn replace_text(g: Digraph, a: &HashMap<&str, &str>) -> Digraph {
    let mut res = g.clone();
    for n in g.node_indices() {
        let mut len = g[n].value.len();
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
        res[n].width = if g[n].dummy { 1 } else { len + 4 };
        res[n].height = 3;
    }
    res
}

fn permute(g: Digraph, levels: &[usize]) -> Digraph {
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
        // print_layer(&inner.0, levels.len());
        if inner.1 < outer_optimal_crossings {
            outer_optimal_crossings = inner.1;
            outer_optimal = inner.0;
        }
        // println!("Crossing: {}", outer_optimal_crossings);
        outer_loop_cnt += 1;
    }
    outer_optimal
}

fn get_rand_permute_level(g: Digraph, max_level: usize, level_len: usize) -> Digraph {
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
        if g[n].level == max_level {
            res[n].permutation = rand_perm[cur];
            cur += 1;
        }
    }
    res
}

fn cal_crossings(g: &Digraph, max_level: usize) -> usize {
    let mut cnt = 0;
    for l in 1..max_level {
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

fn cal_crossings_levels(g: &Digraph, max_level: usize) -> Vec<usize> {
    let mut cl = vec![0; max_level - 1];
    for l in 1..max_level {
        let mut lines_level: Vec<(usize, usize)> = Vec::new();
        for e in g.edge_indices() {
            if g[g.edge_endpoints(e).unwrap().0].level == l {
                lines_level.push((
                    g[g.edge_endpoints(e).unwrap().0].permutation,
                    g[g.edge_endpoints(e).unwrap().1].permutation,
                ));
            }
        }
        cl[l - 1] += crossings(&lines_level);
    }

    cl
}

fn cal_level(g: &Digraph, i: NodeIndex) -> usize {
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

fn crossings(lines: &[(usize, usize)]) -> usize {
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
struct NodeData {
    level: usize,
    width: usize,
    height: usize,
    pos: (usize, usize),
    value: String,
    dummy: bool,
    permutation: usize,
    temp_perm: f32,
}

#[derive(Debug, Default, PartialEq, Clone)]
struct EdgeData {
    visited: bool,
}

#[derive(Debug, Clone)]
enum Connection {
    Straight {
        from: (usize, usize),
        dummy: (bool, bool),
    },
    Bent {
        x1: usize,
        x2: usize,
        y: isize,
        dummy: (bool, bool),
    },
}

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/dag.pest"]
pub struct DagGraphParser;
