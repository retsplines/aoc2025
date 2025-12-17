use std::io::Lines;

#[derive(Debug)]
struct PresentShape {
    index: usize,
    shape: [bool; 9]
}

#[derive(Debug)]
struct RegionSpec {
    dimensions: (usize, usize),
    quantities: [usize; 6]
}

impl PresentShape {
    fn occupies(&self) -> usize {
        self.shape.iter()
            .filter(|b| **b)
            .count()
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

    let mut lines = contents.lines();

    // List of regions
    let mut regions: Vec<RegionSpec> = vec![];
    let mut present_shapes: Vec<PresentShape> = vec![];

    // Read each line
    loop {

        // Read a header line (like 0:) or a tree definition line (like 4x4: 0 0 0 0 2 0)
        let header_or_tree_line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        // If it's just blank, read another line
        if header_or_tree_line.trim().is_empty() {
            continue;
        }

        // If it contains a colon and an x, it's a region definition
        if header_or_tree_line.contains('x') && header_or_tree_line.contains(':') {

            // Split at the colon
            let mut parts = header_or_tree_line.split(':');
            let dimensions = parts.next().unwrap();
            let quantities = parts.next().unwrap();

            // Break the dimensions
            let x_and_y = dimensions.trim().split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let quantities = quantities.trim().split(' ')
                .map(|qty| qty.trim())
                .map(|qty| qty.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            regions.push(RegionSpec {
                dimensions: (x_and_y[0], x_and_y[1]),
                quantities: quantities[..].try_into().unwrap(),
            });

            continue;
        }

        // If it contains just a colon, it's the header for a present shape
        if header_or_tree_line.contains(':') {

            let header = header_or_tree_line.strip_suffix(':').unwrap_or(header_or_tree_line);
            let mut layout = vec![];
            for i in 0..3 {
                let chunk = lines.next().unwrap();
                layout.extend(chunk.chars().map(|c| c == '#'))
            }

            present_shapes.push(PresentShape {
                index: header.parse().unwrap(),
                shape: layout[0..9].try_into().unwrap(),
            });

            continue;
        }
    }

    let mut regions_that_fit = 0;

    // For each of the region specs, work out if there's enough space by summing
    // the consumed space of each of the shapes in that region spec
    for (region_index, region) in regions.iter().enumerate() {

        let available_space = region.dimensions.0 * region.dimensions.1;
        let consumed_space = region.quantities.iter().enumerate()
            .map(|(shape_index, qty)| qty * present_shapes[shape_index].occupies())
            .sum::<usize>();

        let max_space = region.quantities.iter().enumerate()
            .map(|(_, qty)| qty * 9)
            .sum::<usize>();

        let ratio = available_space as f32 / consumed_space as f32;

        println!(
            "Region {} needs {} max {} has {} (ratio {})",
            region_index, consumed_space, max_space, available_space, ratio
        );

        // Turns out this is doable statistically...
        if ratio < 1.0 {
            continue;
        }

        regions_that_fit += 1;
    }

    println!("Regions that fit: {regions_that_fit}");

}
