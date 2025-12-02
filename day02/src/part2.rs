// oh no you are going to make me use regex aren't you
use fancy_regex::Regex;

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
    let re = Regex::new(r"^(\d+)(\1)+$").expect("failed to compile regexp");

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

            // Check if it's made up of digits repeated twice or more
            if re.is_match(digits.as_str()).expect("failed to match regexp") {
                total += i;
                println!("Found repeated number: {} @ range {}", i, range);
            }
        }

    }

    println!("Total sum of repeated numbers: {}", total);
}