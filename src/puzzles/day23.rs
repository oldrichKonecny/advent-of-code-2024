use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day23;

impl DaySolver<u64> for Day23 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        Ok(count_t_triangles(&input.input))
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let password = lan_party_password(&input.input);
        log::info!("LAN party password: {}", password);
        Ok(password.split(',').count() as u64)
    }
}

/// Node ids encode the two-letter name: id = (a - 'a') * 26 + (b - 'a').
const NODE_COUNT: usize = 26 * 26;

struct Graph {
    adjacent: Vec<Vec<u16>>,
    edges: Vec<(u16, u16)>,
}

fn parse(input: &str) -> Graph {
    let mut adjacent = vec![Vec::new(); NODE_COUNT];
    let mut edges = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let Some((a, b)) = line.split_once('-') else {
            continue;
        };
        let (a, b) = (node_id(a), node_id(b));
        adjacent[a as usize].push(b);
        adjacent[b as usize].push(a);
        edges.push((a.min(b), a.max(b)));
    }
    for neighbors in &mut adjacent {
        neighbors.sort_unstable();
    }
    Graph { adjacent, edges }
}

fn node_id(name: &str) -> u16 {
    let bytes = name.as_bytes();
    (bytes[0] - b'a') as u16 * 26 + (bytes[1] - b'a') as u16
}

fn starts_with_t(id: u16) -> bool {
    id / 26 == (b't' - b'a') as u16
}

fn count_t_triangles(input: &str) -> u64 {
    let graph = parse(input);
    let mut count = 0;
    for &(a, b) in &graph.edges {
        // Common neighbors c with c > b > a, so each triangle is counted once.
        for &c in &graph.adjacent[a as usize] {
            if c > b
                && graph.adjacent[b as usize].binary_search(&c).is_ok()
                && (starts_with_t(a) || starts_with_t(b) || starts_with_t(c))
            {
                count += 1;
            }
        }
    }
    count
}

fn lan_party_password(input: &str) -> String {
    let graph = parse(input);
    let nodes: Vec<u16> = (0..NODE_COUNT as u16)
        .filter(|&n| !graph.adjacent[n as usize].is_empty())
        .collect();

    let mut best = Vec::new();
    bron_kerbosch(&graph, &mut Vec::new(), nodes, Vec::new(), &mut best);

    let mut names: Vec<String> = best.iter().map(|&id| node_name(id)).collect();
    names.sort();
    names.join(",")
}

/// Bron–Kerbosch with pivoting; `p` and `x` are sorted candidate/excluded sets.
fn bron_kerbosch(
    graph: &Graph,
    clique: &mut Vec<u16>,
    mut p: Vec<u16>,
    mut x: Vec<u16>,
    best: &mut Vec<u16>,
) {
    if p.is_empty() && x.is_empty() {
        if clique.len() > best.len() {
            *best = clique.clone();
        }
        return;
    }
    let pivot = p
        .iter()
        .chain(x.iter())
        .copied()
        .max_by_key(|&u| graph.adjacent[u as usize].len())
        .unwrap();
    let candidates: Vec<u16> = p
        .iter()
        .copied()
        .filter(|&v| graph.adjacent[pivot as usize].binary_search(&v).is_err())
        .collect();
    for v in candidates {
        let neighbors = &graph.adjacent[v as usize];
        let next_p = p
            .iter()
            .copied()
            .filter(|n| neighbors.binary_search(n).is_ok())
            .collect();
        let next_x = x
            .iter()
            .copied()
            .filter(|n| neighbors.binary_search(n).is_ok())
            .collect();
        clique.push(v);
        bron_kerbosch(graph, clique, next_p, next_x, best);
        clique.pop();
        p.retain(|&n| n != v);
        let pos = x.binary_search(&v).unwrap_or_else(|e| e);
        x.insert(pos, v);
    }
}

fn node_name(id: u16) -> String {
    let first = (id / 26) as u8 + b'a';
    let second = (id % 26) as u8 + b'a';
    String::from_utf8(vec![first, second]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id_roundtrip() {
        assert_eq!(node_id("aa"), 0);
        assert!(starts_with_t(node_id("tc")));
        assert!(!starts_with_t(node_id("kh")));
        assert_eq!(node_name(node_id("tc")), "tc");
    }
}
