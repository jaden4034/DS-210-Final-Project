use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

// I did not require to do other data processing since in the euroroad.csv, there are no missing values, and
// each line in the CSV is expected to contain two unsigned integers separated by a comma,
// representing an edge between two nodes in a graph. Lines that do not conform to this format are ignored
// For example, a line "1,2" means there is an edge between node 1 and node 2
pub fn read_graph_file(path: &str) -> io::Result<(Vec<usize>, Vec<usize>)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split(',').collect();

        if parts.len() != 2 {
            continue; // Ignore invalid lines
        }

        if let Ok(node) = parts[0].parse::<usize>() {
            if let Ok(edge) = parts[1].parse::<usize>() {
                nodes.push(node);
                edges.push(edge);
            }
        }
    }

    Ok((nodes, edges))
}

// Constructing an adjacency list from a list of nodes and edges
// Representing a graph where the adjacency list is a vector of HashSet<usize>, with each HashSet representing the neighbors of a node
// Required to calculate and observe the connectivity, degrees of separation, etc. 
pub fn create_adjacency_list(nodes: &[usize], edges: &[usize]) -> Vec<HashSet<usize>> {
    let max_node = *nodes.iter().max().unwrap();
    let mut adj_list = vec![HashSet::new(); max_node + 1]; // +1 for zero-indexing
    for (&node, &edge) in nodes.iter().zip(edges) {
        if node != edge { // Avoid self connections
            if node < adj_list.len() && edge < adj_list.len() {
                adj_list[node].insert(edge);
                adj_list[edge].insert(node);
            }
        }
    }
    adj_list
}
