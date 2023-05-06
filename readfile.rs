use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Reading a file containing edges of a graph and returning a HashMap representing the adjacency list
pub fn read_file(file_name: &str) -> HashMap<u128, Vec<u128>> {
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    // New empty HashMap to hold the adjacency list
    let mut adjacency_list = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nodes: Vec<_> = line.split_whitespace().collect();
        let node1: u128 = nodes[0].parse().expect("Failed to parse node1");
        let node2: u128 = nodes[1].parse().expect("Failed to parse node2");

        // Adding node2 to the list of neighbors for node1, and vice versa
        adjacency_list.entry(node1).or_insert_with(Vec::new).push(node2);
        adjacency_list.entry(node2).or_insert_with(Vec::new).push(node1);
    }
    adjacency_list
    
}