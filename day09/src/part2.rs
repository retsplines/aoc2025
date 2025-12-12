// Plan of attack for this one...
// Load the red tiles as before.
// Link the tiles up in a chain by edges, including linking the last to the first.
// Separate the edges into horizontal and vertical edges
// Compute all the red-to-red rectangles, sort them by area, largest first.
// Use PiP (Point in Polygon) edge-crossing/ray-casting to find only rectangles where:
//  - all four corners are inside the polygon, and
//  - there are no edges inside the polygon (which could be islands maybe?)


use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct RedTile {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Edge {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64
}

#[derive(Debug)]
struct Rectangle<'a> {
    tiles: (&'a RedTile, &'a RedTile),
    area: i64
}

impl fmt::Display for RedTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Red {},{})", self.x, self.y)
    }
}

impl FromStr for RedTile {
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

impl RedTile {
    /// Area of a rectangle produced with this tile and another
    fn area_of(&self, other: &RedTile) -> i64 {
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

    // List of red tiles
    let red_tiles: Vec<RedTile> = contents.lines().map(|line| line.parse().unwrap()).collect();

    // List of all rectangles
    let mut rectangles: Vec<Rectangle> = vec![];

    // List of all edges
    let mut edges: Vec<Edge> = vec![];

    // Compute all rectangles
    for (tile_index, tile) in red_tiles.iter().enumerate() {

        // Compare with every other tile
        for (other_tile_index, other_tile) in red_tiles.iter().enumerate().skip(tile_index) {

            // Not eligible if it's the same tile!
            if tile_index == other_tile_index {
                continue;
            }

            rectangles.push(Rectangle {
                tiles: (tile, other_tile),
                area: tile.area_of(other_tile),
            })

        }
    }

    // Compute all edges
    for tile_pair in red_tiles.windows(2) {
        edges.push(Edge {
            x1: tile_pair[0].x,
            y1: tile_pair[0].y,
            x2: tile_pair[1].x,
            y2: tile_pair[1].y,
        })
    }

    // Add the first-to-last edge too
    if let (Some(first), Some(last)) = (red_tiles.first(), red_tiles.last()) {
        edges.push(Edge {
            x1: first.x,
            y1: first.y,
            x2: last.x,
            y2: last.y,
        })
    }

    
}
