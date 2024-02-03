use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

type EdgeIndex = usize;
type NodeIndex = usize;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Node {
    first_outgoing_edge: Option<EdgeIndex>,
    heat_cost: usize,
}

#[derive(Default, Clone, Copy)]
struct Edge {
    target: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

#[derive(Default)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, heat_cost: usize) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node {
            first_outgoing_edge: None,
            heat_cost,
        });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: EdgeIndex) {
        let edge_index = self.edges.len();
        let node = &mut self.nodes[source];
        self.edges.push(Edge {
            target,
            next_edge: node.first_outgoing_edge,
        });
        node.first_outgoing_edge = Some(edge_index);
    }
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut graph = Graph::new();
    let mut node_index_matrix: Vec<Vec<NodeIndex>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        node_index_matrix.push(line.chars().map(|c| graph.add_node(c as usize)).collect());
    }
    for i in 0..node_index_matrix.len() {
        for j in 0..node_index_matrix[0].len() {
            let source = node_index_matrix[i][j];
            if i > 0 {
                graph.add_edge(source, node_index_matrix[i - 1][j]);
            }
            if i + 1 < node_index_matrix.len() {
                graph.add_edge(source, node_index_matrix[i + 1][j]);
            }
            if j > 0 {
                graph.add_edge(source, node_index_matrix[i][j - 1]);
            }
            if i + 1 < node_index_matrix[0].len() {
                graph.add_edge(source, node_index_matrix[i][j + 1]);
            }
        }
    }
    let mut unvisited_nodes: Vec<Option<&Node>> = graph.nodes.iter().map(|n| Some(n)).collect();
    let mut previous_nodes: Vec<usize> = vec![0; graph.nodes.len()];
    let mut shortest_path: Vec<usize> = vec![usize::MAX; graph.nodes.len()];
    shortest_path[0] = 0;
    while unvisited_nodes.iter().all(|n| n.is_some()) {
        let (index, smallest_distance_node): (usize, &Option<&Node>) = unvisited_nodes
            .iter()
            .enumerate()
            .min_by(|(i, _), (j, _)| shortest_path[*i].cmp(&shortest_path[*j]))
            .unwrap();
        if let Some(node) = smallest_distance_node {
            let mut next_edge_index = node.first_outgoing_edge;
            while let Some(edge_index) = next_edge_index {
                let temp_distance =
                    shortest_path[index] + graph.nodes[graph.edges[edge_index].target].heat_cost;
                if temp_distance < shortest_path[edge_index] {
                    shortest_path[edge_index] = temp_distance;
                    previous_nodes[edge_index] = index;
                }
                next_edge_index = graph.edges[edge_index].next_edge;
            }
            unvisited_nodes[index] = None;
        }
    }
    print_results(&previous_nodes, &shortest_path, 0, graph.nodes.len() - 1);

    0
}

fn print_results(
    previous_nodes: &Vec<usize>,
    shortest_path: &Vec<usize>,
    start_node: usize,
    target_node: usize,
) {
    let mut path: Vec<usize> = vec![];
    let mut node = target_node;
    while node != start_node {
        path.push(node);
        node = previous_nodes[node]
    }
    path.push(start_node);
    path.reverse();
    println!(
        " Found the following best path with a value of {}",
        shortest_path[target_node]
    );
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt"), 102);
}
