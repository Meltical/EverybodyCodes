mod quest_12;
mod quest_13;
mod quest_14;
mod quest_18;
mod coord;

use crate::quest_12::solve_all as quest_12;
use crate::quest_13::solve_all as quest_13;
use crate::quest_14::solve_all as quest_14;
use crate::quest_18::solve_all as quest_18;
use std::fs::File;
use std::io;
use std::io::BufRead;

/// Read a matrix from a file, y x
pub fn read_matrix(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).unwrap();
    let mut m: Vec<Vec<char>> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        m.push(row);
    }
    m
}

pub fn read_vector(path: &str) -> Vec<(i32, i32)> {
    let file = File::open(path).unwrap();
    let mut v: Vec<(i32, i32)> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut positions = line.split_whitespace();
        let x = positions.next().unwrap().parse().unwrap();
        let y = positions.next().unwrap().parse().unwrap();
        v.push((x, y))
    }
    v
}

fn main() {
    quest_12();
    quest_13();
    quest_14();
    quest_18();
}
