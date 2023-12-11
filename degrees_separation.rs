use std::collections::{HashSet, VecDeque};

// Computing the connectivity of each node in a graph
// Utilizing Breadth-First Search (BFS) to determine how many nodes are reachable from each starting node 
pub fn bfs_connectivity(adj_list: &[HashSet<usize>]) -> Vec<usize> {
    let mut connectivity = vec![0; adj_list.len()];

    for start in 0..adj_list.len() {
        let mut visited = vec![false; adj_list.len()];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj_list[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        connectivity[start] = visited.iter().filter(|&&v| v).count();
    }

    // Returning a vector where each element indicates the number of nodes reachable from the corresponding starting node
    connectivity
}

// A helper function for BFS
// Finding the shortest path lengths from a given start node to all other nodes in the graph
// Returning a vector where each element is the shortest path length from the start node to the node at that index 
// If a node is unreachable, the distance is -1
fn bfs_shortest_path_all(adj_list: &[HashSet<usize>], start: usize) -> Vec<i32> {
    let mut distances = vec![-1; adj_list.len()];
    let mut queue = VecDeque::new();

    distances[start] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj_list[node] {
            if distances[neighbor] == -1 {
                distances[neighbor] = distances[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    distances
}


/// Calculating the average shortest path length for all pairs of vertices using bfs_shortest_path all function
pub fn average_shortest_path_length(adj_list: &[HashSet<usize>]) -> f64 {
    let mut total_distance = 0;
    let mut path_count = 0;

    for start in 0..adj_list.len() {
        let distances = bfs_shortest_path_all(adj_list, start);
        for distance in distances.iter() {
            if *distance != -1 {
                total_distance += *distance as usize;
                path_count += 1;
            }
        }
    }

    if path_count == 0 { return 0.0; }
    total_distance as f64 / path_count as f64
}

// Calculating how many node pairs have each possible degree of separation and then converts these counts into percentages
pub fn separation_distribution_and_max(adj_list: &[HashSet<usize>]) -> (Vec<f64>, usize, f64) {
    let total_pairs = adj_list.len() * (adj_list.len() - 1) / 2;
    let mut distribution = vec![0usize; adj_list.len()]; // Using the number of nodes as the potential max degree

    // Computing distribution of degrees of separation
    // Returning the most common degree of separation and its percentage
    for start in 0..adj_list.len() {
        let distances = bfs_shortest_path_all(adj_list, start);
        for &distance in distances.iter() {
            if distance > 0 && (distance as usize) < adj_list.len() {
                distribution[distance as usize] += 1;
            }
        }
    }

    // Converting counts to percentages
    let percent_distribution: Vec<f64> = distribution.iter()
        .map(|&count| 100.0 * count as f64 / total_pairs as f64)
        .collect();

    // Find the degree of separation with the maximum percentage
    let (max_degree_of_separation, &max_percentage) = percent_distribution.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    (percent_distribution, max_degree_of_separation, max_percentage)
}

// Calculating statistical measures of the separation distances between nodes in the graph
// Computing the mean (average) and standard deviation of the separation distances, providing insights of overall connectivity
pub fn separation_statistics(adj_list: &[HashSet<usize>]) -> (f64, f64) {
    let mut sum = 0f64;
    let mut sum_sq = 0f64;
    let mut count = 0f64;

    for start in 0..adj_list.len() {
        let distances = bfs_shortest_path_all(adj_list, start);
        for &distance in distances.iter() {
            if distance >= 0 {
                sum += distance as f64;
                sum_sq += (distance as f64).powi(2);
                count += 1.0;
            }
        }
    }

    let mean = sum / count;
    let variance = (sum_sq - sum.powi(2) / count) / count;
    let std_dev = variance.sqrt();

    (mean, std_dev)
}