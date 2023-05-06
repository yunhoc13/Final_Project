use std::collections::{HashMap, VecDeque};
use rand::seq::IteratorRandom;

pub struct Graph {
    pub adj_list: HashMap<u128, Vec<u128>>,
}

// Creating a new Graph instance with the adjacency list
impl Graph {
    pub fn new(adj_list: HashMap<u128, Vec<u128>>) -> Self {
        Self { adj_list }
    }

    // Performing BFS algorithm to find the shortest path between start and end node
    pub fn bfs(&self, start_node: u128, end_node: u128) -> Option<usize> {
        let mut visited: HashMap<u128, usize> = HashMap::new();
        let mut queue: VecDeque<u128> = VecDeque::new();

        visited.insert(start_node, 0);
        queue.push_back(start_node);

        while let Some(node) = queue.pop_front() {
            // Returning the path length
            if node == end_node {
                return Some(visited[&node]);
            }
            
            // Otherwise, adding the unvisited neighbors to the queue
            for neighbor in self.adj_list[&node].iter() {
                if !visited.contains_key(neighbor) {
                    visited.insert(*neighbor, visited[&node] + 1);
                    queue.push_back(*neighbor);
                }
            }
        }

        // If not-reachable, None
        None
    }

    // Selecting random pairs from the adjcancy list (two points)
    pub fn random_pairs(&self, num_pairs: usize) -> Vec<(u128, u128)> {
        self.adj_list
            .keys()
            .choose_multiple(&mut rand::thread_rng(), num_pairs * 2)
            .chunks(2)
            .map(|chunk| (*chunk[0], *chunk[1]))
            .collect()
    }

    /* Performing six degrees of separation test 
        by selecting random pairs and calculating the degrees of separation */
    pub fn six_degrees_of_separation(&self, num_pairs: usize) -> HashMap<usize, Vec<(u128, u128)>> {
        let pairs = self.random_pairs(num_pairs);
        let mut degrees: HashMap<usize, Vec<(u128, u128)>> = HashMap::new();
        
        for pair in pairs {
            if let Some(degree) = self.bfs(pair.0, pair.1) {
                let entry = degrees.entry(degree).or_insert(Vec::new());
                entry.push(pair);
            }
        }
        
        degrees
    }

}
