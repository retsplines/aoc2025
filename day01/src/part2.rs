use std::str::FromStr;

struct Turn (i32);

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        let distance: i32 = distance.parse()
            .map_err(|e| format!("Invalid distance: {}", e))?;

        match direction {
            "L" => Ok(Turn(-distance)),
            "R" => Ok(Turn(distance)),
            _ => Err(format!("Invalid turn direction: {}", s))
        }
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
        
        let mut turn = line.parse::<Turn>().unwrap().0;
        println!("Rotating {}", turn);

        // Find how many full rotations the turn entails
        // Each full rotation necessitates passing zero exactly once
        let mut zeroes_in_turn = turn.abs() / 100;
        println!("Full rotations {}", zeroes_in_turn);
        turn = turn % 100;

        println!("Rotating (effective) {}", turn);
        
        // Check if we land on zero during this turn
        if turn > 0 {
            if dial < 100 && dial + turn >= 100 {
                zeroes_in_turn += 1;
            }
        } else if turn < 0 {
            if dial > 0 && dial + turn <= 0 {
                zeroes_in_turn += 1;
            }
        }
        
        dial = (dial + turn).rem_euclid(100);

        println!("Turned {}, new dial position: {}, passed zero {}", turn, dial, zeroes_in_turn);

        zeroes += zeroes_in_turn;        

    }

    println!("Dial left pointing at {}", dial);
    println!("Number of times dial hit zero: {}", zeroes);

}
