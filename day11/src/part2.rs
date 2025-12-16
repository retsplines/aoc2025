use std::collections::{HashMap};
use std::str::FromStr;

#[derive(Debug)]
struct Node {
    id: String,
    outputs: Vec<String>,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .split(' ')
            .map(|part| part.trim().strip_suffix(':').unwrap_or(part));

        Ok(Node {
            id: parts.nth(0).unwrap().to_string(),
            outputs: parts.map(|part| part.to_string()).collect(),
        })
    }
}

/// Counts the number of paths from -> in nodes.
fn find<'a>(from: &'a str, to: &str, nodes: &'a HashMap<&str, Vec<String>>, cached_paths: &mut HashMap<&'a str, usize>) -> usize {

    if cached_paths.contains_key(from) {
        return *cached_paths.get(from).unwrap();
    }

    let mut paths = 0usize;

    for out in nodes[from].iter() {
        if out == to {
            paths += 1;
        } else {
            paths += find(out, to, nodes, cached_paths);
        }
    }

    // Cache the path, avoiding recalculating this segment
    cached_paths.insert(from, paths);

    paths
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

    // HashMap of nodes
    let mut nodes_map: HashMap<&str, Vec<String>> = HashMap::new();

    // Parse the lines into nodes
    let nodes: Vec<Node> = lines.map(|line| line.parse::<Node>().unwrap()).collect();
    for node in nodes.iter() {
        nodes_map.insert(node.id.as_str(), node.outputs.clone());

        // Make sure all outputs are added as nodes too, initially with no outputs themselves
        for output in node.outputs.iter() {
            if !nodes_map.contains_key(output.as_str()) {
                nodes_map.insert(output.as_str(), Vec::new());
            }
        }
    }
    println!("{}", nodes.iter().count());

    println!("Looking for SVR->FFT paths");
    let svr_to_fft = find("svr", "fft", &nodes_map, &mut HashMap::new());
    println!("Looking for FFT->DAC paths");
    let fft_to_dac = find("fft", "dac", &nodes_map, &mut HashMap::new());
    println!("Looking for DAC->OUT paths");
    let dac_to_out = find("dac", "out", &nodes_map, &mut HashMap::new());

    // Result will be the multiple of these
    println!("Result: {svr_to_fft}*{fft_to_dac}*{dac_to_out}={}", svr_to_fft * fft_to_dac * dac_to_out);
}
