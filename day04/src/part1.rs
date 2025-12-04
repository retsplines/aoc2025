mod part2;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Build a vec of rows of vecs of columns
    let mut rows: Vec<Vec<bool>> = vec![];

    // Iterate over each line in the file
    for line in contents.lines() {

        // Read the line as a row
        rows.push(line.chars().map(|c| c == '@').collect())

    }

    // Count movable rolls
    let mut movable = 0;

    // Now find all the rolls that have fewer than four adjacent rolls
    for (row_index, row) in rows.iter().enumerate() {

        'roll: for (col_index, &col) in row.iter().enumerate() {

            // Ignore non-rolls
            if !col {
                continue;
            }

            let left_bound = (col_index as isize - 1).max(0);
            let top_bound = (row_index as isize - 1).max(0);
            let right_bound = (col_index as isize + 1).min(row.len() as isize - 1);
            let bottom_bound = (row_index as isize + 1).min(rows.len() as isize - 1);

            println!("Scanning {}.{}, bounds = left {} top {} right {} bottom {}", row_index, col_index, left_bound, top_bound, right_bound, bottom_bound);

            // Count adjacent rolls
            let mut adjacent = 0;

            for searched_row_index in top_bound..=bottom_bound {
                for searched_col_index in left_bound..=right_bound {

                    // Ignore the current item
                    if searched_row_index == row_index as isize && searched_col_index == col_index as isize {
                        continue;
                    }

                    println!("    Search {}-{}", searched_row_index, searched_col_index);

                    if rows[searched_row_index as usize][searched_col_index as usize] {
                        adjacent += 1;
                        println!("      Found adjacent");

                        if adjacent >= 4 {
                            println!("Has reached 4 adjacent - not movable.");
                            continue 'roll;
                        }
                    }
                }
            }

            println!("Only has {} adjacent - movable", adjacent);
            movable += 1;
        }
    }

    println!("{:?}", movable);

}
