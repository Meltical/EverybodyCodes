use crate::read_matrix;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
    value: char,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point ({}, {}), '{}'", self.x, self.y, self.value)
    }
}

#[derive(Eq, PartialEq)]
struct Cell {
    point: Point,
    cost: i32,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_start(m: &Vec<Vec<char>>, part3: bool) -> Point {
    let starts = vec!['S', 'E'];
    let start_value = starts[part3 as usize];
    let mut start = Point {
        x: 0,
        y: 0,
        value: start_value,
    };
    for (y, row) in m.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == start_value {
                start.x = x as i32;
                start.y = y as i32;
                break;
            }
        }
    }
    start
}

fn find_adjacent_points(m: &Vec<Vec<char>>, p: Point) -> Vec<Point> {
    let is_correct = |point: &mut Point| {
        if point.x >= m[0].len() as i32 || point.y >= m.len() as i32 {
            return false;
        }
        if point.x < 0 || point.y < 0 {
            return false;
        }
        point.value = m[point.y as usize][point.x as usize];
        if point.value == '#' {
            return false;
        }
        return true;
    };
    let mut adjacent_points: Vec<Point> = Vec::new();
    let mut top = Point {
        x: p.x,
        y: p.y - 1,
        value: ' ',
    };
    if is_correct(&mut top) {
        adjacent_points.push(top);
    }
    let mut right = Point {
        x: p.x + 1,
        y: p.y,
        value: ' ',
    };
    if is_correct(&mut right) {
        adjacent_points.push(right);
    }
    let mut bottom = Point {
        x: p.x,
        y: p.y + 1,
        value: ' ',
    };
    if is_correct(&mut bottom) {
        adjacent_points.push(bottom);
    }
    let mut left = Point {
        x: p.x - 1,
        y: p.y,
        value: ' ',
    };
    if is_correct(&mut left) {
        adjacent_points.push(left);
    }
    adjacent_points
}

fn compute_cost(p1: Point, p2: Point) -> i32 {
    let start = p1.value.to_digit(10).unwrap_or(0) as i32;
    let end = p2.value.to_digit(10).unwrap_or(0) as i32;
    let abs = (end - start).abs();
    if abs > 5 {
        11 - abs
    } else {
        abs + 1
    }
}

fn bfs(m: &Vec<Vec<char>>, part3: bool) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let start = find_start(m, part3);
    heap.push(Cell {
        point: start,
        cost: 0,
    });

    let ends = vec!['E', 'S'];
    while let Some(cell) = heap.pop() {
        if !visited.insert(cell.point) {
            continue;
        }
        if cell.point.value == ends[part3 as usize] {
            return cell.cost;
        }

        let points = find_adjacent_points(m, cell.point);
        for point in points {
            heap.push(Cell {
                point,
                cost: cell.cost + compute_cost(cell.point, point),
            });
        }
    }

    eprintln!("No path to 'E' found.");
    -1
}

pub fn solve_all() {
    println!("Quest 13:");
    let m = read_matrix("inputs/quest_13/part_1.txt");
    println!("Part 1: {}", bfs(&m, false));
    let m = read_matrix("inputs/quest_13/part_2.txt");
    println!("Part 2: {}", bfs(&m, false));
    let m = read_matrix("inputs/quest_13/part_3.txt");
    println!("Part 3: {}", bfs(&m, true));
}
