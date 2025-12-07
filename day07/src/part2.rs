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
    let mut tachyons: HashMap<usize, i64> = HashMap::new();

    // Obtain the first line
    let mut lines = contents.lines();
    let first_line = lines.next().expect("No first line found");

    // Set the tachyon starting position
    tachyons.insert(first_line.find("S").unwrap(), 1);

    // Iterate over the rest of the lines
    for line in lines {

        // Build the next set of tachyon position -> timeline counts
        let mut next_tachyons: HashMap<usize, i64> = HashMap::new();

        // For each of the current tachyon positions...
        for (&tachyon_pos, &timeline_count) in tachyons.iter() {

            // If there's a splitter at this position
            if line.chars().nth(tachyon_pos).unwrap() == '^' {

                // Left and right will now each receive all the timelines,
                // ...including any they already had
                next_tachyons.insert(
                    tachyon_pos - 1,
                    next_tachyons.get(&(tachyon_pos - 1)).unwrap_or(&0) + timeline_count
                );

                next_tachyons.insert(
                    tachyon_pos + 1,
                    next_tachyons.get(&(tachyon_pos + 1)).unwrap_or(&0) + timeline_count
                );

            } else {

                // Otherwise, it's just a continuation so just pass the timelines down
                next_tachyons.insert(
                    tachyon_pos,
                    next_tachyons.get(&tachyon_pos).unwrap_or(&0) + timeline_count
                );

            }

        }

        // Carry the tachyons forward to the next line
        tachyons = next_tachyons;
    }

    // Sum all the tachyons in the final row
    let total = tachyons.values().fold(0, |acc, &v| acc + v);
    println!("Timelines: {}", total);
}
