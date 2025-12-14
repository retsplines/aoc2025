use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem, Variable};

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

    // Total presses required
    let mut total = 0;

    // For each machine
    for (machine_index, machine) in machines.iter().enumerate() {

        let mut problem = Problem::new(OptimizationDirection::Minimize);

        // Create an integer variable for each button that can be pressed 0->infinity times
        let button_vars: Vec<Variable> = machine.buttons.iter()
            .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
            .collect();

        // Add a constraint for each of the joltage values
        for (joltage_index, joltage_target) in machine.joltages.iter().enumerate() {
            let mut expr = LinearExpr::empty();

            // For each button, if it contributes to this joltage, make the constraint dependent on it
            machine.buttons.iter().enumerate()
                .filter(|(_, button)| button.0.contains(&joltage_index))
                .for_each(|(i, _)| expr.add(button_vars[i], 1.0));

            // Add the constraint for this joltage target
            // Ultimately this means "vary the variables I added to expr to target joltage_target"
            // (along with all the other constraints that will be added for the other joltage targets)
            problem.add_constraint(expr, ComparisonOp::Eq, *joltage_target as f64);
        }

        // Solve!
        let result = problem.solve().unwrap();
        println!("Machine {machine_index} requires {}", result.objective().round());
        total += result.objective().round() as i32;
    }

    println!("Total: {total}");
}
