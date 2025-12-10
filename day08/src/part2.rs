// this is pure trash

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    circuit: u32
}

impl fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(JB {},{},{} Circ. {})", self.x, self.y, self.z, self.circuit)
    }
}

impl FromStr for JunctionBox {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split into the three coordinates
        let mut parts = s.split(",");

        Ok(Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
            circuit: 0
        })
    }
}

impl JunctionBox {

    /// Euclidean distance between two 3D points:
    ///
    /// d(p, q) = sqrt((px - qx)² + (py - qy)² + (pz - qz)²)
    fn distance_to(&self, other: &JunctionBox) -> i64 {
        ((((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt() * 1000.0) as i64
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
    let lines = contents.lines();

    // Process each line, parsing the comma-separated values into 3D points
    let mut points: Vec<JunctionBox> = vec![];
    for (line_num, line) in lines.enumerate() {
        let mut node: JunctionBox = line.parse().unwrap();
        node.circuit = line_num as u32;
        points.push(node);

    }

    let mut distances: Vec<(i64, usize, usize)> = Vec::new();

    // For each point
    for (point_index, point) in points.iter().enumerate() {

        // Compare with every other point
        for (other_point_index, other_point) in points.iter().enumerate().skip(point_index) {

            // Not eligible if it's the same point!
            if point_index == other_point_index {
                continue;
            }

            let pair_distance = point.distance_to(other_point);

            distances.push((
                pair_distance,
                point_index,
                other_point_index
            ));

        }
    }

    // Sort the list by distance
    distances.sort_by(|a, b| a.0.cmp(&b.0));

    for (run, pair) in distances.iter().enumerate() {

        println!("Run {}", run);
        println!("Next pair: {} & {} distance {}", points[pair.1], points[pair.2], pair.0);

        // If they're on the same circuit, skip it
        if points[pair.1].circuit != points[pair.2].circuit {

            // Update any points on the old circuit number to the new one
            let old_cn = points[pair.2].circuit;
            let new_cn = points[pair.1].circuit;
            for point in points.iter_mut() {
                if point.circuit == old_cn {
                    println!("Updating {point} to circuit {new_cn}");
                    point.circuit = new_cn;
                }
            }
        }

        // If all items are in the same circuit, exit
        let all_in_same = points.iter()
            .map(|p| p.circuit).collect::<Vec<u32>>()
            .windows(2).all(|w| w[0] == w[1]);

        if all_in_same {
            println!("All on same circuit! - {:?} with {:?} = {}", points[pair.1], points[pair.2], points[pair.1].x * points[pair.2].x);
            break;
        }

    }

    // Count up the circuits
    let mut circuit_counts: HashMap<u32, u32> = HashMap::new();
    for point in points.iter() {
        let mut existing_count = *circuit_counts.get(&point.circuit).unwrap_or(&0);
        existing_count += 1;
        circuit_counts.insert(point.circuit, existing_count);
    }

    let total: usize = circuit_counts.values().count();

    println!("{:?} total circuits {}", circuit_counts, total);


    // Find the 3 maximums
    let mut maximums: Vec<u32> = circuit_counts.values().into_iter().map(|x| x.clone()).collect();
    maximums.sort();
    println!("{:?}", maximums.into_iter().rev().take(3).reduce(|v, acc| acc * v ));

}
