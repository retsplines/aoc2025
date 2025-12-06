mod part2;

use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Multiply,
    Add
}

impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Multiply => left * right,
            Operator::Add => left + right
        }
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err(format!("Unknown operator {}", s))
        }
    }
}

#[derive(Debug)]
enum Line {
    Operands(Vec<i64>),
    Operators(Vec<Operator>)
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Split into a vector of substrings first
        let cols: Vec<&str> = s.split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        // Operator or Operands line?
        match cols[0].chars().next() {
            None => Err(String::from("No columns")),
            Some('*' | '+') => Ok(Self::Operators(
                cols.iter().map(|s| s.parse().expect("Invalid operator")).collect()
            )),
            Some('0'..'9') => Ok(Self::Operands(
                cols.iter().map(|s| s.parse().expect("Invalid operand")).collect()
            )),
            Some(s) => Err(format!("Unknown column value {}", s))
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

    let mut operands: Vec<Vec<i64>> = vec![];

    // Process each line
    for line in contents.lines() {

        let line: Line = line.parse().expect("Bad line");
        println!("Line: {:?}", line);

        match line {

            Line::Operands(line_operands) => operands.push(line_operands),
            Line::Operators(line_operators) => {

                // Reduce our operands, applying the operators
                let totals = operands.into_iter().reduce(|acc, next| {

                    // Start with the existing total
                    let mut subtotal = acc.clone();

                    for (op_index, op) in line_operators.iter().enumerate() {

                        subtotal[op_index] = op.apply(next[op_index], acc[op_index])

                    }

                    subtotal

                });

                // Sum the totals
                let sum: i64 = totals.expect("No values to total").into_iter().sum();

                println!("Totals: {}", sum);

                // Do not process any more lines at this point
                break

            }
        }

    }


}
