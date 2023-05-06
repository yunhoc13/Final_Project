mod bfs;
mod readfile;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use maplit::hashmap;

fn main() {
    let file_name = "gplus_combined.txt";
    let adjacency_list = readfile::read_file(file_name);
    let graph = bfs::Graph::new(adjacency_list);

    // `num_pairs` can be modified upto 15000000, but it takes up a lot of tims to run
    let num_pairs = 10; // Number of pairs chosen from txt file
    let degrees_of_separation = graph.six_degrees_of_separation(num_pairs);

    println!("Degrees of Separation:");
    for (degree, pairs) in &degrees_of_separation {
        println!("Degree {}: {} pairs", degree, pairs.len());
        }



// Test statements
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_adj_list() {
        let adj_list = readfile::read_file("test.txt");
        let mut expected_adj_list = HashMap::new();
        expected_adj_list.insert(1, vec![2, 3, 4]);
        expected_adj_list.insert(2, vec![1, 3]);
        expected_adj_list.insert(3, vec![1, 2, 4]);
        expected_adj_list.insert(4, vec![1, 3, 5]);
        expected_adj_list.insert(5, vec![4, 6]);
        expected_adj_list.insert(6, vec![5, 7]);
        expected_adj_list.insert(7, vec![6, 8]);
        expected_adj_list.insert(8, vec![7]);

        assert_eq!(adj_list, expected_adj_list);
    }

    #[test]
    fn test_bfs() {
        let adjacency_list = hashmap! {
            1 => vec![2, 3, 4],
            2 => vec![1, 3],
            3 => vec![1, 2, 4],
            4 => vec![1, 3],
        };
        let graph = bfs::Graph::new(adjacency_list);

        assert_eq!(graph.bfs(1, 4), Some(1));
        assert_eq!(graph.bfs(1, 5), None);
        assert_eq!(graph.bfs(2, 3), Some(1));
        assert_eq!(graph.bfs(2, 5), None);
    }

    #[test]
    fn test_six_degrees_of_separation() {
        let adj_list = readfile::read_file("test.txt");
        let graph = bfs::Graph::new(adj_list);

        let num_pairs = 10;
        let degrees = graph.six_degrees_of_separation(num_pairs);

        for (degree, pairs) in degrees.iter() {
            for pair in pairs {
                let (start_node, end_node) = *pair;

                if let Some(visited_nodes) = graph.bfs(start_node, end_node) {
                    assert_eq!(*degree, visited_nodes);

                } else {
                    // Skipping current pair if bfs() fails to find a path
                    continue;
                }
            }
        }
    }

}
