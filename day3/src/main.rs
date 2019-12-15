use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

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

fn challenge(mut input: impl Iterator<Item = String>) -> Option<i32> {
    if let (Some(str1), Some(str2)) = (input.next(), input.next()) {
        // transform from string into WireVec enums
        let wire1: Vec<WireVec> = str1.split(',').map(WireVec::new).collect();
        let wire2: Vec<WireVec> = str2.split(',').map(WireVec::new).collect();

        // transform from WireVec enums into Line enums
        let line1: Vec<Line> = wirevecs_to_lines(wire1);
        let line2: Vec<Line> = wirevecs_to_lines(wire2);

        let (mut h_lines, mut v_lines) = separate_and_sort_lines(line1);
        let h_tree = SegmentTree::new(h_lines.clone());
        let v_tree = SegmentTree::new(v_lines.clone());
        h_lines.sort_by_key(|l| l.get_perpendicular_coordinate());
        v_lines.sort_by_key(|l| l.get_perpendicular_coordinate());

        let mut min_dist_opt: Option<i32> = None;
        for line in line2 {
            match line {
                Line::Horizontal {
                    y_coordinate,
                    x_start,
                    x_end,
                } => {
                    let possible_y_intersects = v_tree.query(y_coordinate);
                    let mut start = v_lines
                        .binary_search_by_key(&x_start, |l| {
                            l.get_perpendicular_coordinate()
                        })
                        .unwrap_or_else(|x| x);
                    let mut end = v_lines
                        .binary_search_by_key(&x_end, |l| {
                            l.get_perpendicular_coordinate()
                        })
                        .unwrap_or_else(|x| x);
                    if start > end {
                        let tmp = start;
                        start = end;
                        end = tmp;
                    }
                    let A: HashSet<&Line, RandomState> =
                        HashSet::from_iter(possible_y_intersects.iter());
                    let B = HashSet::from_iter(v_lines[start..end].iter());
                    let result = A.intersection(&B);
                    println!("h_line: {} {} {}", x_start, x_end, y_coordinate);
                    println!("y_intersects: {:?}", A);
                    println!("x_intersects: {:?}", B);
                    println!("result: {:?}", result);
                    result.for_each(|x| match x {
                        Line::Vertical { x_coordinate, .. } => {
                            let dist = y_coordinate.abs() + x_coordinate.abs();
                            if let Some(min_dist) = min_dist_opt {
                                if dist < min_dist {
                                    min_dist_opt = Some(dist)
                                }
                            } else {
                                min_dist_opt = Some(dist)
                            }
                        }
                        _ => {}
                    });
                }
                Line::Vertical {
                    x_coordinate,
                    y_start,
                    y_end,
                } => {
                    let possible_x_intersects = h_tree.query(x_coordinate);
                    let mut start = h_lines
                        .binary_search_by_key(&y_start, |l| {
                            l.get_perpendicular_coordinate()
                        })
                        .unwrap_or_else(|x| x);
                    let mut end = h_lines
                        .binary_search_by_key(&y_end, |l| {
                            l.get_perpendicular_coordinate()
                        })
                        .unwrap_or_else(|x| x);
                    if start > end {
                        let tmp = start;
                        start = end;
                        end = tmp;
                    }
                    let A: HashSet<&Line, RandomState> =
                        HashSet::from_iter(possible_x_intersects.iter());
                    let B = HashSet::from_iter(h_lines[start..end].iter());
                    let result = A.intersection(&B);
                    println!("v_line: {} {} {}", y_start, y_end, x_coordinate);
                    println!("x_intersects: {:?}", A);
                    println!("y_intersects: {:?}", B);
                    println!("result: {:?}", result);
                    result.for_each(|x| match x {
                        Line::Horizontal { y_coordinate, .. } => {
                            let dist = x_coordinate.abs() + y_coordinate.abs();
                            if let Some(min_dist) = min_dist_opt {
                                if dist < min_dist {
                                    min_dist_opt = Some(dist)
                                }
                            } else {
                                min_dist_opt = Some(dist)
                            }
                        }
                        _ => {}
                    });
                }
            }
        }

        min_dist_opt
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

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
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

    fn get_interval(&self) -> Interval {
        match self {
            Line::Vertical { y_start, y_end, .. } => Interval::new(*y_start, *y_end),
            Line::Horizontal { x_start, x_end, .. } => {
                Interval::new(*x_start, *x_end)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Interval {
        if start > end {
            Interval {
                start: end,
                end: start,
            }
        } else {
            Interval { start, end }
        }
    }

    fn contains(&self, rhs: &Interval) -> bool {
        self.start <= rhs.start && rhs.end <= self.end
    }

    fn intersects(&self, rhs: &Interval) -> bool {
        (self.start <= rhs.start && rhs.start <= self.end)
            || (self.start <= rhs.end && rhs.end <= self.end)
    }

    fn contains_point(&self, pt: i32) -> bool {
        self.start < pt && pt < self.end
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
    interval: Interval,
    lines: Vec<Line>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            interval: Interval { start: 0, end: 0 },
            lines: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct SegmentTree {
    tree: Vec<Node>,
}

impl SegmentTree {
    pub fn new(segments: Vec<Line>) -> SegmentTree {
        // find elementary intervals
        let mut sorted_points = Vec::with_capacity(2 * segments.len());
        for line in &segments {
            let Interval { start, end } = line.get_interval();
            if let Err(idx) = sorted_points.binary_search(&start) {
                sorted_points.insert(idx, start);
            }
            if let Err(idx) = sorted_points.binary_search(&end) {
                sorted_points.insert(idx, end);
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
            n_internal + n_leaves
        ];

        // If the leaf nodes are split amongst the two lower levels of a
        // complete binary tree, we start by filling out the level furthest from
        // the root first followed by the level above it. The level furthest
        // from the root starts `split` entries into the array, and we then wrap
        // around at `n_leaves` to start filling in the level above. This
        // maintains the invariant that leaf nodes, when viewed left to right in
        // the resulting binary tree, form partitions of the entire number line
        // in order.
        let split = n_leaves.next_power_of_two() - n_leaves;
        let mut prev: i32 = std::i32::MIN;

        for (i, point) in sorted_points.iter().enumerate() {
            let open_int = Node {
                interval: Interval::new(prev, *point),
                ..Default::default()
            };
            let closed_int = Node {
                interval: Interval::new(*point, *point),
                ..Default::default()
            };
            prev = *point;

            let leaf_index_1 = (split + 2 * i) % n_leaves;
            let leaf_index_2 = (split + 2 * i + 1) % n_leaves;
            tree[n_internal + leaf_index_1] = open_int;
            tree[n_internal + leaf_index_2] = closed_int;

            Self::update_parents(
                &mut tree,
                n_internal + leaf_index_1,
                UpdateParent::Either,
            );
            Self::update_parents(
                &mut tree,
                n_internal + leaf_index_2,
                UpdateParent::Either,
            );
        }

        let open_int = Node {
            interval: Interval::new(prev, std::i32::MAX),
            ..Default::default()
        };

        let leaf_index = (split + 2 * sorted_points.len()) % n_leaves;
        tree[n_internal + leaf_index] = open_int;

        Self::update_parents(
            &mut tree,
            n_internal + leaf_index,
            UpdateParent::Either,
        );

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
                    tree[parent_i].interval.start = tree[child_i].interval.start;
                    Self::update_parents(tree, parent_i, opt);
                }
            }
            UpdateParent::OnlyRight => {
                if !is_left_child {
                    tree[parent_i].interval.end = tree[child_i].interval.end;
                    Self::update_parents(tree, parent_i, opt);
                }
            }
            UpdateParent::Either => {
                if is_left_child {
                    tree[parent_i].interval.start = tree[child_i].interval.start;
                    Self::update_parents(
                        tree,
                        parent_i,
                        UpdateParent::OnlyLeft,
                    );
                } else {
                    tree[parent_i].interval.end = tree[child_i].interval.end;
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
        let interval = line.get_interval();
        if let Some(node) = tree.get_mut(root) {
            if interval.contains(&node.interval) {
                node.lines.push(line);
            } else {
                let left_child_i = 2 * root + 1;
                if let Some(left_child) = tree.get(left_child_i) {
                    if interval.intersects(&left_child.interval) {
                        Self::insert(line, tree, left_child_i);
                    }
                }
                let right_child_i = 2 * root + 2;
                if let Some(right_child) = tree.get(right_child_i) {
                    if interval.intersects(&right_child.interval) {
                        Self::insert(line, tree, right_child_i);
                    }
                }
            }
        }
    }

    pub fn query(&self, p: i32) -> Vec<Line> {
        let mut ret = Vec::new();
        Self::_query(p, &self.tree, 0, &mut ret);
        ret
    }

    fn _query(p: i32, tree: &Vec<Node>, root: usize, ret: &mut Vec<Line>) {
        if let Some(root_node) = tree.get(root) {
            if root_node.interval.contains_point(p) {
                ret.extend_from_slice(&root_node.lines);
                Self::_query(p, tree, 2 * root + 1, ret);
                Self::_query(p, tree, 2 * root + 2, ret);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec!["R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()];
        assert_eq!(challenge(input.into_iter()), Some(6));
    }

    #[test]
    fn test_segment_tree2() {
        let input = vec![
            WireVec::Right(8),
            WireVec::Up(5),
            WireVec::Left(5),
            WireVec::Down(3),
        ];
        let line1 = wirevecs_to_lines(input);
        let (mut h_lines, mut v_lines) = separate_and_sort_lines(line1);
        println!("{:?}", h_lines);
        let h_tree = SegmentTree::new(h_lines.clone());

        let result = h_tree.query(6);
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test2() {
        let input = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
            "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
        ];
        assert_eq!(challenge(input.into_iter()), Some(159));
    }

    #[test]
    fn test3() {
        let input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
        ];
        assert_eq!(challenge(input.into_iter()), Some(135));
    }

    #[test]
    fn test_segment_tree() {
        let line1 = Line::Horizontal {
            x_start: 0,
            x_end: 10,
            y_coordinate: 0,
        };
        let line2 = Line::Horizontal {
            x_start: 5,
            x_end: 15,
            y_coordinate: 0,
        };

        let segments = vec![line1.clone(), line2.clone()];
        let tree = SegmentTree::new(segments);

        let q = tree.query(-1);
        assert!(q.is_empty());

        let q = tree.query(1);
        assert_eq!(q.len(), 1);
        assert!(q.contains(&line1));

        let q = tree.query(6);
        assert_eq!(q.len(), 2);
        assert!(q.contains(&line1));
        assert!(q.contains(&line2));

        let q = tree.query(11);
        assert_eq!(q.len(), 1);
        assert!(q.contains(&line2));

        let q = tree.query(20);
        assert!(q.is_empty());
    }
}
