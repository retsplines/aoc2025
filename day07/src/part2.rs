// the previous approach was fun, but won't complete until the heat-death of the universe for the
// new requirements, so I'm going with a different plan here...

use std::collections::HashMap;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Maintain a HashMap of the column => the number of timelines in which a Tachyon is present
    // at that column. Each row only computes the new Tachyon counts.
    let tachyons: HashMap<usize, i32> = HashMap::new();

    todo!();
}
