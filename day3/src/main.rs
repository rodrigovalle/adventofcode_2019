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
    println!("{:?}", challenge(input));
}

fn find_array_dimensions(wire: &Vec<WireVec>) -> (i32, i32, i32, i32) {
    let (mut min_x, mut max_x) = (0, 0);
    let (mut min_y, mut max_y) = (0, 0);
    let (mut x, mut y) = (0, 0);
    for dir in wire {
        match dir {
            WireVec::Up(i) => y += i,
            WireVec::Down(i) => y -= i,
            WireVec::Left(i) => x -= i,
            WireVec::Right(i) => x += i,
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
    }

    (min_x, max_x, min_y, max_y)
}

fn challenge(mut input: impl Iterator<Item = String>) -> Option<i32> {
    let str1 = input.next().unwrap();
    let str2 = input.next().unwrap();

    let wire1 = str1.split(',').map(WireVec::new).collect();
    let wire2: Vec<WireVec> = str2.split(',').map(WireVec::new).collect();

    // create 2d array
    let (min_x, max_x, min_y, max_y) = find_array_dimensions(&wire1);
    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;
    let mut arr = vec![(false, 0); width * height];

    // plot wire 1
    // start at (0, 0) in array coordinates
    let (mut x, mut y) = (-min_x, -min_y);
    let mut steps = 0;
    for dir in wire1 {
        let (move_x, move_y) = match dir {
            WireVec::Up(d) => (0, d),
            WireVec::Down(d) => (0, -d),
            WireVec::Left(d) => (-d, 0),
            WireVec::Right(d) => (d, 0),
        };

        let (end_x, end_y) = (x + move_x, y + move_y);
        let (dx, dy) = (move_x.signum(), move_y.signum());
        loop {
            arr[width * y as usize + x as usize] = (true, steps);
            if x == end_x && y == end_y {
                break;
            }
            x += dx;
            y += dy;
            steps += 1;
        }
    }

    // print_array
    // for j in (0..height).rev() {
    //     for i in 0..width {
    //         if arr[(width * j) + i] {
    //             print!("1");
    //         } else {
    //             print!("0");
    //         }
    //     }
    //     println!();
    // }

    // plot wire 2
    // start at (0, 0) in array coordinates
    let (mut x, mut y) = (-min_x, -min_y);
    let mut steps = 0;
    let mut steps_min = None;
    for dir in wire2 {
        let (move_x, move_y) = match dir {
            WireVec::Up(d) => (0, d),
            WireVec::Down(d) => (0, -d),
            WireVec::Left(d) => (-d, 0),
            WireVec::Right(d) => (d, 0),
        };

        let (dx, dy) = (move_x.signum(), move_y.signum());
        let (end_x, end_y) = (x + move_x, y + move_y);
        loop {
            // check for intersections with wire 1
            let i = width as i32 * y + x;
            if let Some((set, wire1_steps)) = arr.get(i as usize) {
                if *set {
                    let dist = steps + wire1_steps;
                    if dist != 0 {
                        steps_min = match steps_min {
                            Some(cur_min) if dist < cur_min => Some(dist),
                            None => Some(dist),
                            _ => steps_min,
                        }
                    }
                }
            }
            if x == end_x && y == end_y {
                break;
            }
            x += dx;
            y += dy;
            steps += 1;
        }
    }

    steps_min
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    //#![feature(trace_macros)]
    //trace_macros!(true);
    use super::*;

    macro_rules! str_vec {
        ( $($s:expr),* $(,)? ) => ( vec![$($s.to_string()),*] )
    }

    #[test]
    fn test1() {
        let input = str_vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        assert_eq!(challenge(input.into_iter()), Some(610));
    }

    #[test]
    fn test2() {
        let input = str_vec!["R8,U5,L5,D3", "U7,R6,D4,L4"];
        assert_eq!(challenge(input.into_iter()), Some(30));
    }

    fn test3() {
        let input = str_vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        assert_eq!(challenge(input.into_iter()), Some(410));
    }
}
