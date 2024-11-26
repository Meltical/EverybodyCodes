use crate::{read_matrix, read_vector};

fn get_hit_position(initial_position: (i32, i32)) -> (i32, i32) {
    (
        initial_position.0 / 2,
        initial_position.1 - initial_position.0 / 2 - initial_position.0 % 2,
    )
}

fn solve(m: &mut Vec<Vec<char>>, part: u8) -> i32 {
    let mut score = 0;
    let origin_y = m.len() - 1;
    for x in 1..m[0].len() {
        for y in 0..m.len() {
            if m[y][x] != '.' {
                let rank = get_rank_3((x as i32, y as i32), origin_y as i32);
                if part == 1 {
                    score += rank;
                } else {
                    score += rank * (1 << ((m[y][x] == 'H') as u8));
                }
            }
        }
    }
    score
}

fn get_rank_3(hit_position: (i32, i32), origin_y: i32) -> i32 {
    let distance = (origin_y - hit_position.1).abs() + hit_position.0;
    let letter = (distance % 3) + 1;
    let power = distance / 3;
    // println!("({},{}): distance: {}, letter: {}, power: {}", hit_position.0, hit_position.1, distance, letter, power);
    letter * power
}

fn get_ranks(hit_position: (i32, i32)) -> i32 {
    for base in 0..3 {
        let y = hit_position.1 - base;
        let horizontal = hit_position.0 + y;

        if hit_position.0 < y {
            continue;
        }
        if hit_position.0 <= 2 * y {
            return (base + 1) * y;
        }
        if horizontal % 3 == 0 {
            return (base + 1) * (horizontal / 3);
        }
    }
    unreachable!()
}

fn solve_part_3(initial_positions: Vec<(i32, i32)>) -> i32 {
    let mut score = 0;
    for (x, y) in initial_positions {
        let hit_position = get_hit_position((x, y));
        let rank = get_ranks(hit_position);
        score += rank;
    }
    score
}

pub fn solve_all() {
    println!("Quest 12:");
    let mut m1 = read_matrix("inputs/quest_12/part_1.txt");
    println!("Part 1 : {}", solve(&mut m1, 1));
    let mut m2 = read_matrix("inputs/quest_12/part_2.txt");
    println!("Part 2 : {}", solve(&mut m2, 2));
    let positions = read_vector("inputs/quest_12/part_3.txt");
    println!("Part 3 : {}", solve_part_3(positions));
}
