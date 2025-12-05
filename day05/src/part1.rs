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

enum State {
    ReadingRanges,
    ReadingIngredients
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
    let mut state = State::ReadingRanges;
    let mut fresh = 0u32;

    // Read each line
    for line in contents.lines() {

        // Moving onto ingredients?
        if line.len() == 0 {
            state = State::ReadingIngredients;
            continue;
        }

        // What are we currently doing?
        match state {

            State::ReadingRanges => {
                ranges.push(line.parse().expect("Bad range"));
                println!("Got range {:?}", ranges.last().unwrap())
            },

            State::ReadingIngredients => {

                let ingredient: u64 = line.parse().expect("Bad ingredient");

                // Check if it's valid
                for range in ranges.iter() {
                    if ingredient >= range.start && ingredient <= range.end {
                        fresh += 1;
                        break;
                    }
                }

            }
        }
    }

    println!("There are {} fresh", fresh);
}
