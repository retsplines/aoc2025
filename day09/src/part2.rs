// Plan of attack for this one...
// Load the red tiles as before. [/]
// Link the tiles up in a chain by edges, including linking the last to the first. [/]
// Compute all the red-to-red rectangles, sort them by area, largest first. [/]
// Separate the edges into horizontal and vertical edges [/]
// Use PiP (Point in Polygon) edge-crossing/ray-casting to find only rectangles where:
//  - all four corners are inside the polygon, and
//  - there are no edges inside the polygon (which could be islands maybe?)

use std::cmp::{max, min};
use std::fmt;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
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
    tiles: (&'a Point, &'a Point),
    area: i64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Red {},{})", self.x, self.y)
    }
}

impl FromStr for Point {
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

impl Point {
    /// Area of a rectangle produced with this tile and another
    fn area_of(&self, other: &Point) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

/// Decide if an (x, y) point is inside a polygon composed of vertical and horizontal edges.
/// This works by checking if we pass an odd number of edges in each of {x, y} to reach the point.
fn point_in_poly(point: &Point, edges_h: &[&Edge], edges_v: &[&Edge]) -> bool {

    let horizontal_edges_above_point: Vec<&Edge> = edges_h.iter()
        .filter(|edge| edge.y1 <= point.y && edge.x1 <= point.x && edge.x2 >= point.x)
        .copied()
        .collect();

    let horizontal_edges_below_point: Vec<&Edge> = edges_h.iter()
        .filter(|edge| edge.y1 >= point.y && edge.x1 <= point.x && edge.x2 >= point.x)
        .copied()
        .collect();

    let vertical_edges_to_left_of_point: Vec<&Edge> = edges_v.iter()
        .filter(|&edge| edge.x1 <= point.x && edge.y1 <= point.y && edge.y2 >= point.y)
        .copied()
        .collect();

    let vertical_edges_to_right_of_point: Vec<&Edge> = edges_v.iter()
        .filter(|&edge| edge.x1 >= point.x && edge.y1 <= point.y && edge.y2 >= point.y)
        .copied()
        .collect();

    // Debug
    println!("Point {point:?}: ");
    for horizontal in horizontal_edges_above_point.iter() {
        println!("  Up   {:?}", horizontal);
    }
    for horizontal in horizontal_edges_below_point.iter() {
        println!("  Down {:?}", horizontal);
    }
    for vertical in vertical_edges_to_left_of_point.iter() {
        println!("  Left   {:?}", vertical);
    }
    for vertical in vertical_edges_to_right_of_point.iter() {
        println!("  Rght {:?}", vertical);
    }

    horizontal_edges_above_point.len() % 2 == 1 && horizontal_edges_below_point.len() % 2 == 1 &&
        vertical_edges_to_left_of_point.len() % 2 == 1 && vertical_edges_to_right_of_point.len() % 2 == 1
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
    let red_tiles: Vec<Point> = contents.lines().map(|line| line.parse().unwrap()).collect();

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

    // Sort the rectangles by the largest area
    rectangles.sort_by(|a, b| a.area.cmp(&b.area).reverse());

    // Compute all edges
    for tile_pair in red_tiles.windows(2) {
        edges.push(Edge {
            x1: min(tile_pair[0].x, tile_pair[1].x),
            y1: min(tile_pair[0].y, tile_pair[1].y),
            x2: max(tile_pair[0].x, tile_pair[1].x),
            y2: max(tile_pair[0].y, tile_pair[1].y)
        });

        println!("{:?}", edges.last());
    }

    // Add the first-to-last edge too
    if let (Some(first), Some(last)) = (red_tiles.first(), red_tiles.last()) {
        edges.push(Edge {
            x1: min(first.x, last.x),
            y1: min(first.y, last.y),
            x2: max(first.x, last.x),
            y2: max(first.y, last.y)
        })
    }

    // Filter into horizontal and vertical edges
    // Horizontal edges have y values that are equal
    // Vertical edges have x values that are equal
    let edges_horizontal: Vec<&Edge> = edges.iter().filter(|edge| edge.y1 == edge.y2).collect();
    let edges_vertical: Vec<&Edge> = edges.iter().filter(|edge| edge.x1 == edge.x2).collect();

    // Iterate through the largest rectangles
    'rect: for rectangle in rectangles.iter() {

        // Find the "normalised" bounds of the rectangle
        let left = min(rectangle.tiles.0.x, rectangle.tiles.1.x) + 1;
        let right = max(rectangle.tiles.0.x, rectangle.tiles.1.x) - 1;
        let top = min(rectangle.tiles.0.y, rectangle.tiles.1.y) + 1;
        let bottom = max(rectangle.tiles.0.y, rectangle.tiles.1.y) - 1;

        // Test to see if a point inside the rectangle is inside the polygon
        if !point_in_poly(& Point {x: left + 1, y: top + 1 }, &edges_horizontal, &edges_vertical) {
            println!("Ruled out {:?} because point inside not inside loop", rectangle);
            continue;
        }

        // Check if any edges of the polygon intersect the rectangle
        for edge in edges_horizontal.iter() {
            let edge_left = min(edge.x1, edge.x2);
            let edge_right = max(edge.x1, edge.x2);
            let edge_y = edge.y1;
            if edge_y >= top && edge_y <= bottom && !(edge_right < left || edge_left > right) {
                println!("Ruled out {:?} because h-edge {:?} intersects it", rectangle, edge);
                continue 'rect;
            }
        }

        for edge in edges_vertical.iter() {
            let edge_top = min(edge.y1, edge.y2);
            let edge_bottom = max(edge.y1, edge.y2);
            let edge_x = edge.x1;
            if edge_x >= left && edge_x <= right && !(edge_bottom < top || edge_top > bottom) {
                println!("Ruled out {:?} because v-edge {:?} intersects it", rectangle, edge);
                continue 'rect;
            }
        }

        println!("Found largest rectangle: {:?}", rectangle);
        break;
    }

}
