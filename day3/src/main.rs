use std::collections::HashSet;
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
    println!("{}", challenge(input));
}

fn challenge(mut input: impl Iterator<Item = String>) -> i32 {
    if let (Some(str1), Some(str2)) = (input.next(), input.next()) {
        // transform from string into WireVec enums
        let wire1: Vec<WireVec> = str1.split(',').map(WireVec::new).collect();
        let wire2: Vec<WireVec> = str2.split(',').map(WireVec::new).collect();

        // transform from WireVec enums into Line enums
        let line1: Vec<Line> = wirevecs_to_lines(wire1);
        let line2: Vec<Line> = wirevecs_to_lines(wire2);

        let (h_lines, v_lines) = separate_and_sort_lines(line1);

        let h_tree = SegmentTree::new(h_lines);
        let v_tree = SegmentTree::new(v_lines);
        println!("{:#?}", h_tree);
        0
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

#[derive(Copy, Clone, Debug)]
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

impl Line {
    fn get_perpendicular_coordinate(&self) -> i32 {
        match self {
            Line::Vertical { x_coordinate, .. } => *x_coordinate,
            Line::Horizontal { y_coordinate, .. } => *y_coordinate,
        }
    }

    fn get_segment(&self) -> (&i32, &i32) {
        match self {
            Line::Vertical { y_start, y_end, .. } => (y_start, y_end),
            Line::Horizontal { x_start, x_end, .. } => (x_start, x_end),
        }
    }
}

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

fn insert_sorted(e: Line, sorted: &mut Vec<Line>) {
    let target = e.get_perpendicular_coordinate();
    let idx = sorted
        .binary_search_by_key(&target, |l| l.get_perpendicular_coordinate())
        .unwrap_or_else(|x| x);

    sorted.insert(idx, e)
}

// Separate vertical lines from horizontal lines.
// Sort vertical lines by x coordinate and horizontal lines by y coordinate.
// Return (horizontal_lines, vertical_lines)
fn separate_and_sort_lines(lines: Vec<Line>) -> (Vec<Line>, Vec<Line>) {
    let mut sorted_horiz = Vec::with_capacity(lines.len());
    let mut sorted_vert = Vec::with_capacity(lines.len());

    for line in lines {
        match line {
            h @ Line::Horizontal { .. } => insert_sorted(h, &mut sorted_horiz),
            v @ Line::Vertical { .. } => insert_sorted(v, &mut sorted_vert),
        }
    }

    (sorted_horiz, sorted_vert)
}

enum UpdateParent {
    OnlyLeft,
    OnlyRight,
    Either,
}

#[derive(Clone, Debug)]
struct Node {
    int_start: i32,
    int_end: i32,
    lines: Vec<Line>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            int_start: 0,
            int_end: 0,
            lines: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct SegmentTree {
    tree: Vec<Node>,
}

impl SegmentTree {
    fn new(segments: Vec<Line>) -> SegmentTree {
        // find elementary intervals
        let mut sorted_points = Vec::with_capacity(2 * segments.len());
        for line in &segments {
            let (start, end) = line.get_segment();
            if let Err(idx) = sorted_points.binary_search(start) {
                sorted_points.insert(idx, *start);
            }
            if let Err(idx) = sorted_points.binary_search(end) {
                sorted_points.insert(idx, *end);
            }
        }

        let mut tree = Self::construct_tree(sorted_points);
        for line in segments {
            Self::insert(line, &mut tree, 0);
        }

        SegmentTree { tree }
    }

    // construct a balanced binary tree
    // each leaf represents an interval on the number line
    // n_leaves: an interval between each point and intervals to +/-inf
    // n_internal: # internal nodes in a binary tree is equal to leaves - 1
    fn construct_tree(sorted_points: Vec<i32>) -> Vec<Node> {
        let n_leaves = 2 * sorted_points.len() + 1;
        let n_internal = n_leaves - 1;
        let mut tree = vec![
            Node {
                ..Default::default()
            };
            n_internal
        ];

        let mut prev: i32 = std::i32::MIN;
        for (i, point) in sorted_points.iter().enumerate() {
            let open_int = Node {
                int_start: prev,
                int_end: *point,
                ..Default::default()
            };
            let closed_int = Node {
                int_start: *point,
                int_end: *point,
                ..Default::default()
            };
            prev = *point;
            tree.push(open_int);
            tree.push(closed_int);
            Self::update_parents(
                &mut tree,
                n_internal + 2 * i,
                UpdateParent::Either,
            );
            Self::update_parents(
                &mut tree,
                n_internal + 2 * i + 1,
                UpdateParent::Either,
            );
        }
        let open_int = Node {
            int_start: prev,
            int_end: std::i32::MAX,
            ..Default::default()
        };
        tree.push(open_int);

        tree
    }

    // given a child node, recurse up the tree to the root setting intervals
    // appropriately given the child's interval
    //
    // i.e. if the first node is a completely filled out child node, and it is
    // the left child of its parent node, then the parent node will have the
    // child's left interval bound. We can recurse on the parent so long as the
    // parent is the left child of its own parent, stopping as soon as we switch
    // to being the right child. Eventually, we visit and set every node's
    // interval in the tree.
    fn update_parents(tree: &mut Vec<Node>, child_i: usize, opt: UpdateParent) {
        let parent_i = match child_i.checked_sub(1) {
            Some(n) => n / 2,
            None => return, // underflow
        };
        let is_left_child = ((child_i - 1) % 2) == 0;

        match opt {
            UpdateParent::OnlyLeft => {
                if is_left_child {
                    tree[parent_i].int_start = tree[child_i].int_start;
                    Self::update_parents(tree, parent_i, opt);
                }
            }
            UpdateParent::OnlyRight => {
                if !is_left_child {
                    tree[parent_i].int_end = tree[child_i].int_end;
                    Self::update_parents(tree, parent_i, opt);
                }
            }
            UpdateParent::Either => {
                if is_left_child {
                    tree[parent_i].int_start = tree[child_i].int_start;
                    Self::update_parents(
                        tree,
                        parent_i,
                        UpdateParent::OnlyLeft,
                    );
                } else {
                    tree[parent_i].int_end = tree[child_i].int_end;
                    Self::update_parents(
                        tree,
                        parent_i,
                        UpdateParent::OnlyRight,
                    );
                }
            }
        }
    }

    fn insert(line: Line, tree: &mut Vec<Node>, root: usize) {
        let (start, end) = line.get_segment();
        if let Some(node) = tree.get_mut(root) {
            if node.int_start < *start && *end < node.int_end {
                node.lines.push(line);
            } else {
                let left_child = 2 * root + 1;
                let right_child = 2 * root + 2;
                Self::insert(line, tree, left_child);
                Self::insert(line, tree, right_child);
            }
        }
    }

    fn query(&self, p: i32) -> Vec<Line> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec!["R8,U5,L5,D3", "U7,R6,D4,L4"];
        assert_eq!(challenge(input.iter()), 6);
    }

    #[test]
    fn test2() {
        let input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        assert_eq!(challenge(input.iter()), 135);
    }
}
