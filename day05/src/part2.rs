use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// Merge two ranges into one, if possible
fn merge(left: &Range, right: &Range) -> Option<Range> {

    // If the left range starts before the right range starts
    // [left--------?
    //     [right---?
    if left.start <= right.start {

        // Right entirely contained by left
        // [----left----]
        //    [-right-]
        if right.end <= left.end {
            Some(left.clone())

        // Left ends before right starts, so no intersection at all
        // [---left---]
        //              [---right---
        } else if left.end < right.start {
            None

        // Intersecting
        // [---left---]
        //    [---right---]
        } else {
            Some(Range { start: left.start, end: right.end })
        }

    } else {
        // Try the other way around too
        merge(right, left)
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

        ranges.push(line.parse().expect("Bad range"));
    }

    // Sort the ranges by start
    ranges
        .sort_by(|a, b| a.partial_cmp(b)
        .expect("Unsortable range"));

    let mut unique_ranges = vec![];

    let remaining = ranges.into_iter().reduce(|mut acc, next| {

        // Try to merge with the next item
        match merge(&acc, &next) {

            // Either fully contained or intersected
            Some(standalone_or_merged) => {
                acc = standalone_or_merged;
            },

            // No intersection, so it's a unique range
            None => {
                unique_ranges.push(acc);

                // Start again with this range
                acc = next;
            }
        }

        acc

    }).unwrap_or(Range { start: 0, end: 0 });

    // Add the remaining accumulator
    unique_ranges.push(remaining);

    println!("Ranges {:?}", unique_ranges);

    // Sum up the ranges
    let total = unique_ranges.iter()
        .map(|r| r.end - r.start + 1)
        .sum::<u64>();

    println!("{}", total);
}
