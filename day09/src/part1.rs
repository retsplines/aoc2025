mod part2;

use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct Tile {
    x: i64,
    y: i64,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Tile {},{})", self.x, self.y)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split into the three coordinates
        let mut parts = s.split(",");
        Ok(Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap()
        })
    }
}

impl Tile {
    /// Area of a rectangle produced with this tile and another
    fn area_of(&self, other: &Tile) -> i64 {
        (self.x - other.x + 1).abs() * (self.y - other.y + 1).abs()
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

    // Process each line, parsing the comma-separated values into 3D points
    let tiles: Vec<Tile> = contents.lines().map(|line| line.parse().unwrap()).collect();
    let mut largest_area: i64 = 0;
    let mut largest_pair: Option<(&Tile, &Tile)> = None;

    // For each tile
    for (tile_index, tile) in tiles.iter().enumerate() {

        // Compare with every other tile        // Compare with every other tile
        for (other_tile_index, other_tile) in tiles.iter().enumerate().skip(tile_index) {

            // Not eligible if it's the same tile!
            if tile_index == other_tile_index {
                continue;
            }

            let area = tile.area_of(other_tile);
            if area > largest_area {
                largest_area = area;
                largest_pair = Some((tile, other_tile));
            }

        }
    }

    // Count up the circuits
    println!("Largest area: {largest_area} ({largest_pair:?})");
}
