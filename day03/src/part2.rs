fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <batts>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let batts_needed = args[2].parse::<usize>().unwrap();
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut total_joltage = 0i64;

    // Read each line
    for line in contents.lines() {

        // Extract the digits from the line
        let digits: Vec<i64> = line.chars()
            .map(|c| c.to_digit(10).expect("digit invalid").try_into().unwrap())
            .collect();

        // Calculate the maximum value in this bank
        let mut activated = vec![];
        let mut leftmost = 0usize;

        for want in (0..batts_needed).rev() {

            let range = leftmost..(digits.len() - want);

            // Set our search range
            let search = &digits[range];
            println!("Searching for {} more numbers in range {}-{} ({:?})", want, leftmost, (digits.len() - want), search);

            // Find the largest digit in this range
            let mut max_index = 0;
            let mut max = 0;

            for (idx, &val) in search.iter().enumerate() {
                if val > max {
                    max = val;
                    max_index = leftmost + idx;
                }
            }

            // Add the digit to the found list
            activated.push(max);
            println!("Added {} from index {}", max, max_index);

            // Re-frame the search
            leftmost = max_index + 1;
        }

        println!("Result: {:?} (len: {})", activated, activated.len());

        // Turn the value into an integer
        let joltage = activated
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &v)| v * 10_i64.pow(i as u32))
            .sum::<i64>();

        println!("Joltage of bank: {}", joltage);
        total_joltage = total_joltage + joltage;
    }

    println!("Total Joltage: {}", total_joltage);

}