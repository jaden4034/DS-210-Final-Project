mod degrees_separation;
mod data_preprocessing;

use std::error::Error;
use data_preprocessing::{read_graph_file, create_adjacency_list}; 
use degrees_separation::{bfs_connectivity, average_shortest_path_length, separation_distribution_and_max, separation_statistics};

fn main() -> Result<(), Box<dyn Error>> {
    
    // calling the file and the values inside will be destructured into nodes and edges
    let (nodes, edges) = read_graph_file("euroroad.csv")?;
    
    //  a list where each element represents a node and contains a list of all the nodes it's connected to (its neighbors)
    let adj_list = create_adjacency_list(&nodes, &edges);

    // indicating how many nodes are reachable from the corresponding node in the graph
    let connectivity = bfs_connectivity(&adj_list);

    // calculating the average length of the shortest paths between all pairs of nodes in the graph
    let avg_path_length = average_shortest_path_length(&adj_list);

    // a count of how many node pairs have each possible degree of separation
    let (mut distribution, max_degree_of_separation, max_percentage) = separation_distribution_and_max(&adj_list);
    
    let actual_max_degree_of_separation = 63;

    // a process removing zeroes after the max non-zero separation of distribution
    if let Some(last_non_zero) = distribution.iter().rposition(|&x| x != 0.0) {
        distribution.truncate(last_non_zero + 1);
    }

    // a summary of how far apart nodes are in the graph, on average, and the variability of that separation
    let (mean, std_dev) = separation_statistics(&adj_list);
    
    // Output the results
    // Separation Distribution tells us the percentage of node pairs that have each possible degree of separation, from 1 to a certain maximum. 
    // In the connectivity, 1 represents that the road is isolated or not connected to the others.
    println!("Connectivity: {:?}", connectivity);
    println!("----------------");
    println!("Average Shortest Path Length: {}", avg_path_length);
    println!("----------------");
    println!("Separation Distribution (up to {} degrees): {:?}", actual_max_degree_of_separation, distribution);
    println!("----------------");
    println!("Degree of Separation with the maximum percentage: {}", max_degree_of_separation);
    println!("Maximum percentage of valid connections: {}", max_percentage);
    println!("Mean of Separations: {}", mean);
    println!("Standard Deviation of Separations: {}", std_dev);

    Ok(())
}

// Test Functions

#[test]
fn test_self_connection() {
    let nodes = vec![1, 2, 3];
    let edges = vec![1, 1, 2, 3]; // Self connection at node 1
    let adj_list = create_adjacency_list(&nodes, &edges);
    assert_eq!(adj_list[1].contains(&1), false); // Self connections should not be included
}

#[test]
fn test_no_connection() {
    let nodes = vec![1, 2, 3];
    let edges = vec![1, 2]; // No connection to node 3
    let adj_list = create_adjacency_list(&nodes, &edges);
    assert_eq!(adj_list[2].is_empty(), true); // Node 3 should have no connections
}

#[test]
fn test_second_degree_of_separation() {
   
    let nodes = vec![1, 2, 3, 4, 5];
    let _edges = vec![1, 2, 2, 3, 3, 4, 4, 5]; 

    // Removing direct connections to node 1, isolating second degree connections so first degree is not considered
    let modified_edges = vec![2, 3, 3, 4, 4, 5];

    let adj_list = create_adjacency_list(&nodes, &modified_edges);

    // Using bfs_connectivity to find how many nodes are reachable from node 2
    let connectivity = bfs_connectivity(&adj_list);

    // Check if the connectivity from node 2 matches the expected value
    // The expected value is 3 since, from node 2, nodes 3, 4, and 5 should be reachable
    assert_eq!(connectivity[1], 3, "Test failed: Incorrect number of second-degree separations");
}