use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Turn {
    direction: Direction,
    distance: i32
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Invalid turn direction: {}", s))
        }
    }
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        Ok(Turn {
            direction: direction.parse()?,
            distance: distance.parse()
                .map_err(|e| format!("Invalid distance: {}", e))?
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

    // Iterate over each line in the file
    let mut dial = 50;
    let mut zeroes = 0;

    for line in contents.lines() {
        
        let turn: Turn = line.parse().unwrap();

        // The turn distance is unbounded, so wrapping arithmetic is used
        dial = match turn.direction {
            Direction::Left => (dial - turn.distance).rem_euclid(100),
            Direction::Right => (dial + turn.distance).rem_euclid(100)
        };
        
        println!("{:?}, new dial position: {}", turn, dial);

        if dial == 0 {
            zeroes += 1;
        }
    }

    println!("Number of times dial hit zero: {}", zeroes);

}
