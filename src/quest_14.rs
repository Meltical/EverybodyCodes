use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn step(&self, step: &Step) -> Point {
        let mut point = self.clone();
        match step.direction {
            Direction::U => point.y += step.count,
            Direction::D => point.y -= step.count,
            Direction::R => point.x += step.count,
            Direction::L => point.x -= step.count,
            Direction::F => point.z += step.count,
            Direction::B => point.z -= step.count,
        }
        point
    }

    pub fn add(&self, point: &Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        }
    }

    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point ({}, {}, {})", self.x, self.y, self.z)
    }
}

struct Line {
    current: Point,
    end: Point,
    step: Point, //Vector
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        let direction_x = b.x.cmp(&a.x) as i32;
        let direction_y = b.y.cmp(&a.y) as i32;
        let direction_z = b.z.cmp(&a.z) as i32;
        Line {
            current: a,
            end: b,
            step: Point {
                x: direction_x,
                y: direction_y,
                z: direction_z,
            },
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }
        self.current = self.current.add(&self.step);
        Some(self.current)
    }
}


#[derive(Debug)]
enum Direction {
    U, //upward
    D, //downward
    R, //right
    L, //left
    F, //forward
    B, //backward
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            "F" => Ok(Direction::F),
            "B" => Ok(Direction::B),
            _ => Err(()),
        }
    }
}

struct Step {
    direction: Direction,
    count: i32,
}

impl Debug for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Step ({:?}, {})", self.direction, self.count)
    }
}

fn read_input(path: &str) -> Vec<Vec<Step>> {
    let file = File::open(path).unwrap();
    let mut inputs: Vec<Vec<Step>> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut steps = Vec::new();
        for step in line.split(',') {
            let (direction, count) = step.split_at(1);
            steps.push(Step {
                direction: Direction::from_str(direction).unwrap(),
                count: count.parse().unwrap(),
            })
        }
        inputs.push(steps);
    }
    inputs
}

fn solve_part_1(inputs: &Vec<Step>) -> i32 {
    let mut max: i32 = 0;
    let mut head = Point { x: 0, y: 0, z: 0 };
    for input in inputs {
        head = head.step(&input);
        if head.y > max {
            max = head.y;
        }
    }
    max
}

fn solve_part_2(inputs: &Vec<Vec<Step>>) -> usize {
    let mut visited = HashSet::new();
    for inputs in inputs {
        let mut head = Point { x: 0, y: 0, z: 0 };
        for input in inputs {
            for point in Line::new(head.clone(), head.step(input)) {
                visited.insert(point);
                head = point;
            }
        }
    }
    visited.len()
}

fn solve_part_3(inputs: &Vec<Vec<Step>>) -> usize {
    let mut segments = HashSet::new();
    let mut leaves = HashSet::new();
    for inputs in inputs {
        let mut head = Point { x: 0, y: 0, z: 0 };
        for input in inputs {
            for point in Line::new(head.clone(), head.step(input)) {
                segments.insert(point);
                head = point;
            }
        }
        leaves.insert(head);
    }

    //https://github.com/maneatingape/everybody-codes-rust/blob/main/src/event2024/quest14.rs
    let mut murkiness = HashMap::new();

    for leaf in leaves {
        let mut todo = VecDeque::from([(leaf, 0)]);
        let mut seen = HashSet::from([leaf]);

        while let Some((segment, distance)) = todo.pop_front() {
            let (x, y, z) = (segment.x, segment.y, segment.z);
            let orthogonal = [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ];

            if x == 0 && z == 0 {
                *murkiness.entry(segment).or_insert(0) += distance;
            }

            for next in orthogonal {
                let point = Point::new(next.0, next.1, next.2);
                if segments.contains(&point) && seen.insert(point) {
                    todo.push_back((point, distance + 1));
                }
            }
        }
    }

    *murkiness.values().min().unwrap()
}

pub fn solve_all() {
    println!("Quest 14:");
    let inputs = read_input("inputs/quest_14/part_1.txt");
    println!("Part 1: {:?}", solve_part_1(&inputs[0]));
    let inputs = read_input("inputs/quest_14/part_2.txt");
    println!("Part 2: {:?}", solve_part_2(&inputs));
    let inputs = read_input("inputs/quest_14/part_3.txt");
    println!("Part 3: {:?}", solve_part_3(&inputs));
}
