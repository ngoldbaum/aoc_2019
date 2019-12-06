use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let input = get_contents("input");
    dbg!(checksum(&input));
    dbg!(min_transfers(&input));
    Ok(())
}

fn min_transfers(input: &str) -> usize {
    let orbit_map = get_orbit_map(input);

    let graph = create_tree_graph("COM", &orbit_map);

    let path_you = find_path(&graph, "YOU").unwrap();
    let path_san = find_path(&graph, "SAN").unwrap();

    let mut path: Vec<&str> = Vec::new();

    for el in &path_you {
        if !path_san.contains(el) {
            path.push(el);
        }
    }

    for el in &path_san {
        if !path_you.contains(el) {
            path.push(el);
        }
    }

    path.len() - 2
}

fn find_path(graph: &TreeGraph, destination: &str) -> Option<Vec<String>> {
    let mut paths: Vec<Option<Vec<String>>> = Vec::new();
    if graph.children.len() == 0 {
        return None;
    }
    for child in &(graph.children) {
        if child.node_name == destination {
            paths.push(Some(vec![destination.to_string()]));
        } else {
            paths.push(find_path(&child, destination));
        }
    }
    let mut paths: Vec<Vec<String>> = paths.into_iter().flatten().collect();
    if paths.len() == 0 {
        return None;
    }
    if paths.len() != 1 {
        panic!();
    }
    let path = &mut paths[0];
    path.push((&(graph.node_name)).to_string());
    Some(path.to_vec())
}

fn checksum(input: &str) -> usize {
    let orbit_map = get_orbit_map(input);

    let graph = create_tree_graph("COM", &orbit_map);

    graph_checksum(graph, 0)
}

fn graph_checksum(graph: TreeGraph, level: usize) -> usize {
    let mut checksum = 0;

    for child in graph.children {
        checksum += graph_checksum(child, level + 1)
    }

    checksum + level
}

fn create_tree_graph(root_key: &str, orbit_map: &HashMap<&str, Vec<&str>>) -> TreeGraph {
    let mut graph = TreeGraph {
        node_name: root_key.to_string(),
        children: Vec::new(),
    };

    match orbit_map.get(root_key) {
        Some(children) => {
            for child in children {
                graph.children.push(create_tree_graph(child, orbit_map));
            }
        }
        None => {}
    }

    graph
}

#[derive(Debug)]
struct TreeGraph {
    node_name: String,
    children: Vec<TreeGraph>,
}

fn get_orbit_map(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut ret: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let relationship = line.split(")").collect::<Vec<&str>>();
        let entry = ret.entry(relationship[0]).or_insert(Vec::new());
        entry.push(relationship[1])
    }

    ret
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = get_contents("test_input");
        assert!(checksum(&input) == 42);
    }

    #[test]
    fn part2() {
        let input = get_contents("test_input2");
        assert!(min_transfers(&input) == 4);
    }
}
