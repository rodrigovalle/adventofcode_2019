use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_input(filename: Option<&str>) -> impl Iterator<Item = String> {
    let filename: &str = filename.unwrap_or("input.txt");
    let file = File::open(filename);

    if let Ok(file) = file {
        let reader = io::BufReader::new(file);
        reader.lines().map(|l| l.unwrap())
    } else {
        panic!("Could not open file: {}", filename)
    }
}

fn main() {
    let filename = env::args().nth(1);
    let input = read_input(filename.as_ref().map(String::as_str));
    challenge(input);
}

fn challenge(mut input: impl Iterator<Item = String>) {
    if let (Some(str1), Some(str2)) = (input.next(), input.next()) {
        // transform from string into WireVec enums
        let wire1: Vec<WireVec> = str1.split(',').map(WireVec::new).collect();
        let wire2: Vec<WireVec> = str2.split(',').map(WireVec::new).collect();

        // transform from WireVec enums into Line enums
        let line1: Vec<Line> = wirevecs_to_lines(wire1);
        let line2: Vec<Line> = wirevecs_to_lines(wire2);

        // separate vertical lines from horizontal lines &
        // construct an interval tree for faster queries
        //
        // need to narrow down, e.g.
        //  - vertical lines less/greater than an x coordinate
        //  - vertical lines (ranges) that intersect a y coordinate
        //
        // for line in line1.into_iter() {
        // }
    } else {
        panic!("Not enough wires given");
    }
}

enum WireVec {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl WireVec {
    fn new(input: &str) -> WireVec {
        match input.chars().nth(0) {
            Some('U') => WireVec::Up(input[1..].parse().unwrap()),
            Some('D') => WireVec::Down(input[1..].parse().unwrap()),
            Some('L') => WireVec::Left(input[1..].parse().unwrap()),
            Some('R') => WireVec::Right(input[1..].parse().unwrap()),
            dir => panic!("Unknown direction: {:?}", dir),
        }
    }
}

#[derive(Debug)]
enum Line {
    Vertical {
        x_coordinate: i32,
        y_start: i32,
        y_end: i32,
    },
    Horizontal {
        y_coordinate: i32,
        x_start: i32,
        x_end: i32,
    },
}

// impl Line {
//     fn from_wirevec(&mut coord: (i32, i32), wirevec: WireVec) -> Line {
//     }
// }

fn wirevecs_to_lines(wirevecs: Vec<WireVec>) -> Vec<Line> {
    let (mut x, mut y) = (0, 0);
    wirevecs
        .into_iter()
        .map(move |direction| match direction {
            WireVec::Up(d) => {
                let ret = Line::Vertical {
                    x_coordinate: x,
                    y_start: y,
                    y_end: y + d,
                };
                y = y + d;
                ret
            }
            WireVec::Down(d) => {
                let ret = Line::Vertical {
                    x_coordinate: x,
                    y_start: y,
                    y_end: y - d,
                };
                y = y - d;
                ret
            }
            WireVec::Left(d) => {
                let ret = Line::Horizontal {
                    y_coordinate: y,
                    x_start: x,
                    x_end: x - d,
                };
                x = x - d;
                ret
            }
            WireVec::Right(d) => {
                let ret = Line::Horizontal {
                    y_coordinate: y,
                    x_start: x,
                    x_end: x + d,
                };
                x = x + d;
                ret
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec!["line1", "line2", "line3"];
        // assert_eq!(challenge(input.iter()), <some result>)
    }
}
