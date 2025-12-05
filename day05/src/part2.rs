use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Find the - within the string
        let (start, end) = s.split_at(s.find("-").unwrap());

        Ok(Range {
            start: start.parse().expect("Bad range start"),
            end: end[1..].parse().expect("Bad range end")
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
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut ranges: Vec<Range> = vec![];

    // Read each line
    for line in contents.lines() {

        // Moving onto ingredients (irrelevant in p2)
        if line.len() == 0 {
            break;
        }

        let range: Range = line.parse().expect("Bad range");

        // TODO: Work out if the range extends any existing range, or add it to ranges

    }
}
