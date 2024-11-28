use crate::coord::Coord2;
use crate::read_matrix;
use std::collections::{HashMap, HashSet, VecDeque};

fn count(m: &Vec<Vec<char>>, character: char) -> usize {
    m.iter()
        .map(|row| row.iter().filter(|&&v| v == character).count())
        .sum()
}

fn is_valid(m: &Vec<Vec<char>>, point: Coord2) -> bool {
    if point.x < 0 || point.x >= m[0].len() as i32 {
        return false;
    }
    if point.y < 0 || point.y >= m.len() as i32 {
        return false;
    }
    m[point.y as usize][point.x as usize] != '#'
}

fn find_starts(m: &Vec<Vec<char>>, on_edges: bool) -> HashSet<Coord2> {
    let mut starts = HashSet::new();
    for y in 0..m.len() {
        for x in 0..m[0].len() {
            if on_edges && x != 0 && x != m[0].len() - 1 && y != 0 && y != m.len() - 1 {
                continue;
            }
            if m[y][x] == '.' {
                starts.insert(Coord2 {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    starts
}

fn solve(m: &Vec<Vec<char>>, starts: &HashSet<Coord2>, num_p: usize) -> (i32, i32) {
    let mut num_p = num_p.clone();
    let mut seen = starts.clone();
    let mut todo = VecDeque::from(seen.iter().map(|&c| (c, 0)).collect::<Vec<(Coord2, i32)>>());
    let mut total = 0;

    while let Some((point, distance)) = todo.pop_front() {
        if m[point.y as usize][point.x as usize] == 'P' {
            num_p -= 1;
            total += distance;
            if num_p == 0 {
                return (distance, total);
            }
        }

        let orthogonal = [
            Coord2::new(point.x - 1, point.y),
            Coord2::new(point.x + 1, point.y),
            Coord2::new(point.x, point.y + 1),
            Coord2::new(point.x, point.y - 1),
        ];

        for next in orthogonal {
            if is_valid(m, next) && seen.insert(next) {
                todo.push_back((next, distance + 1))
            }
        }
    }

    unreachable!()
}


// Better solution would be to flood fill from the 'P' cells, incrementing a separate grid of i32 with the cost
// Then the cell '.' with the lowest costs is the closest to all 'P'
fn solve_part_3(m: &Vec<Vec<char>>) -> i32 {
    let num_p = count(&m, 'P');
    let starts = find_starts(&m, false);
    starts
        .iter()
        .map(|&s| solve(m, &HashSet::from([s]), num_p))
        .map(|(_, total)| total)
        .min()
        .unwrap()
}

pub fn solve_all() {
    println!("Quest 18:");
    let m = read_matrix("inputs/quest_18/part_1.txt");
    println!(
        "Part 1: {:?}",
        solve(&m, &find_starts(&m, true), count(&m, 'P')).0
    );
    let m = read_matrix("inputs/quest_18/part_2.txt");
    println!(
        "Part 2: {:?}",
        solve(&m, &find_starts(&m, true), count(&m, 'P')).0
    );
    let m = read_matrix("inputs/quest_18/part_3.txt");
    println!("Part 3: {:?}", solve_part_3(&m));
}
