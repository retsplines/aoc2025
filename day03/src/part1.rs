
fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut total = 0;

    // Read each line
    for line in contents.lines() {

        let mut best_in_bank = 0;

        println!("Bank: {}", line);

        // Read each character of the line
        for (first_index, first_char) in line.chars().enumerate() {

            // Read each character of the line from this point on
            for second_char in line[first_index + 1..].chars() {

                let joltage: i32 = vec![first_char, second_char]
                    .iter().collect::<String>()
                    .parse().expect("Invalid number");

                if joltage > best_in_bank {
                    best_in_bank = joltage;
                }

                println!("First {}, second {}, joltage {}, best so far {}", first_char, second_char, joltage, best_in_bank);
            }
        }

        println!("Best in bank: {}", best_in_bank);

        total += best_in_bank;
    }

    println!("Total: {}", total);
}
