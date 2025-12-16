use std::collections::{HashMap, HashSet};
use std::hash::RandomState;
use std::str::FromStr;
use petgraph::algo;
use petgraph::data::Build;
use petgraph::dot::Dot;
use petgraph::prelude::*;

#[derive(Debug)]
struct Node {
    id: String,
    outputs: Vec<String>,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ')
            .map(|part| part.trim().strip_suffix(':').unwrap_or(part));

        Ok(Node {
            id: parts.nth(0).unwrap().to_string(),
            outputs: parts.map(|part| part.to_string()).collect(),
        })

    }
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Read the file into the graph structure
    let lines = contents.lines();

    // Parse the lines into nodes
    let parsed_nodes: Vec<Node> = lines.map(|line| line.parse::<Node>().unwrap()).collect();

    // Build a Petgraph graph
    let mut graph = DiGraph::<String, ()>::new();

    // Track nodes we've already added
    let mut added_nodes: HashMap<&str, NodeIndex> = HashMap::new();

    for node in parsed_nodes.iter() {

        // Add the source node, which is always new
        if !added_nodes.contains_key(node.id.as_str()) {
            added_nodes.insert(node.id.as_str(), graph.add_node(node.id.clone()));
        }

        // Add the outgoing nodes
        for output in node.outputs.iter() {
            if !added_nodes.contains_key(output.as_str()) {
                added_nodes.insert(output.as_str(), graph.add_node(output.clone()));
            }

            // Add the edge between
            graph.add_edge(added_nodes[node.id.as_str()], added_nodes[output.as_str()], ());
        }

    }

    // Find all the routes between 'you' and 'out'
    let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(
        &graph,
        added_nodes["you"],
        added_nodes["out"],
        0,
        None
    ).collect::<Vec<_>>();

    println!("Total paths: {}", paths.len());

    // Render a dot graph for funsies
    let dot = Dot::new(&graph);
    std::fs::write("part1.dot", format!("{:?}", dot)).unwrap();
}
