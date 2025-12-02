mod part2;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Read comma-separated ranges from the file, they are all on one line
    let ranges: Vec<String> = contents
        .trim()
        .split(',')
        .map(|s| s.parse().expect("parse error (list)"))
        .collect();

    let mut total = 0;

    // Iterate over the ranges
    for (index, range) in ranges.iter().enumerate() {

        // Split the range at -
        let (start, end) = range.split_at(range.find('-').unwrap());
        let start: i64 = start.parse().expect(format!("parse error (start) @ {}", index).as_str());
        let end: i64 = end[1..].parse().expect(format!("parse error (end) @ {}", index).as_str());

        // Iterate over the range
        for i in start..=end {

            // Get the digits of the number as a string
            let digits = i.to_string();

            // If the length is odd, it can't be a repeat
            if digits.len() % 2 != 0 {
                continue;
            }

            // Check if the number is a repeat
            let half_len = digits.len() / 2;
            if &digits[..half_len] == &digits[half_len..] {
                total += i;
            }
        }

    }

    println!("Total sum of repeated numbers: {}", total);
}