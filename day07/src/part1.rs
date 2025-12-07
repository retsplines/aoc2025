fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    // Start with the previous line set to the first, since the first line never changes anyway
    let mut prev_line = lines.next().expect("No first line").to_owned();
    println!("{}", prev_line);

    // Count splits
    let mut splits = 0;

    // Process each line
    for line in lines {

        // Copy this line so that we can make changes to it
        let mut this_line = line.to_owned();

        // Process this line based on the previous one
        for (prev_char_idx, prev_char) in prev_line.chars().enumerate() {

            // Refer to
            let this_char = line.chars().nth(prev_char_idx).expect("No character at index");

            match prev_char {

                // Start, or an incoming beam
                'S' | '|' => {

                    match this_char {

                        // Just continue the beam
                        '.' => this_line.replace_range(prev_char_idx..prev_char_idx + 1, "|"),

                        // Split the beam
                        '^' => {
                            (this_line).replace_range(prev_char_idx - 1..prev_char_idx + 2, "|^|");
                            splits += 1;
                        },

                        // Everything else does nothing
                        _ => {}
                    }

                },

                // Everything else does nothing
                _ => {}

            }

        }

        prev_line = this_line;
        println!("{}", prev_line);

    }

    println!("Splits: {}", splits);

}
