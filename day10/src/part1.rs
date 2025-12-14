use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct Button(Vec<usize>, String);

impl fmt::Display for Button {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for press in self.0.iter() {
            write!(f, "{}", press)?;
        }
        Ok(())
    }
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Strip the outer brackets and parse the inner CSV
        assert!(s.len() > 2, "Empty button list");

        // Map each affected light to a usize then fold into a bitvec with those bits set
        Ok(Button(s[1..s.len() - 1]
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect()
        , s.to_string()))
    }
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<i64>
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Machine [{}] {} {{{}}}",
            self.lights.iter().map(|b| if *b {'#'} else {'.'}).collect::<String>(),
            self.buttons.iter().map(|button| format!("({})", button)).collect::<Vec<String>>().join(" "),
            self.joltages.iter().map(|joltage| format!("{}", joltage)).collect::<Vec<String>>().join(","),
        )
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Split into spaced chunks
        // The first chunk will be the lights spec
        // The last chunk will be the joltages spec
        // Other chunks are buttons
        let parts = s.split(" ").collect::<Vec<&str>>();

        // Must be at least three
        assert!(parts.len() > 2, "Not enough machine descriptor parts");

        // Convert the target lights into a list of booleans
        let lights = parts[0].chars().fold(Vec::new(), |mut acc, c| {
            match c {
                '#' => acc.push(true),
                '.' => acc.push(false),
                _ => ()
            }
            acc
        });

        // Parse the buttons
        let buttons = parts[1..parts.len() - 1].iter()
            .map(|part| { part.parse::<Button>().unwrap() } )
            .collect();

        // Parse the Joltages list
        let joltages_spec = parts.last().unwrap();
        assert!(joltages_spec.len() > 2, "Invalid joltages spec");
        let joltages = joltages_spec[1..joltages_spec.len() - 1].split(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        Ok(Machine {
            lights,
            buttons,
            joltages
        })
    }
}

/// Given a starting state, apply a button press and return the result
fn press(state: &mut [bool], button: &Button) {
    // Flip any bits specified by the button
    for &bit_index in button.0.iter() {
        state[bit_index] = !state[bit_index];
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Process each line, parsing the machine spec on each
    let machines: Vec<Machine> = contents.lines().map(|line| line.parse().unwrap()).collect();

    // Tally total presses
    let mut total_presses = 0;

    // For each machine
    for machine in machines.iter() {

        // Permute the possible button presses
        let mut least_presses = u32::MAX;

        println!("Working on machines {machine:?}");

        for combination in 1..(2u32.pow(machine.buttons.len() as u32)) {

            let mut state = vec![false; machine.lights.len()];

            println!("Trying combination {:b}", combination);

            // Press the relevant buttons
            for (button_index, button) in machine.buttons.iter().enumerate() {
                if (combination >> button_index) & 1 == 1 {
                    println!("Pressing {}", button.1);
                    press(&mut state, button);
                }
            }

            // Matches?
            if state == machine.lights {
                println!("Combination works!");
                if combination.count_ones() <= least_presses {
                    least_presses = combination.count_ones();
                    println!("Best so far: {least_presses}, {combination:b}");
                }
            }
        }

        println!("Best for machine: {least_presses}");
        total_presses += least_presses;

    }

    println!("Least possible: {total_presses}");
}
