// forgive me
use regex::Regex;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Read the last line first, which defines the column widths and operators
    let last_line = contents.lines().last().unwrap();

    // Split into operators, which are *always* left-aligned, so the trailing right-space
    // is meaningful
    let ops: Vec<&str> = Regex::new(r"([+*]\s+)").unwrap()
        .find_iter(last_line)
        .map(|x| x.as_str())
        .collect();

    // Iterate over the ops strings
    let mut col_start: usize = 0;
    let mut grand_total = 0i64;

    for op in ops {

        // Iterate the other lines, extracting the numbers
        let mut numbers: Vec<String> = vec![String::from(""); op.len()];

        for line in contents.lines().take(contents.lines().count() - 1) {
            for (op_col, number) in numbers.iter_mut().enumerate() {
                number.push(line.chars().nth(op_col + col_start).unwrap());
            }
        }

        // Turn the values into integers
        let total = numbers.iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().expect("Bad number"))
            .reduce(|acc, next| match op.trim() {
                "*" => acc * next,
                "+" => acc + next,
                _ => panic!("Bad operator")
            })
            .expect("No values in problem");

        grand_total += total;
        col_start += op.len();
    }

    println!("{}", grand_total);
}
