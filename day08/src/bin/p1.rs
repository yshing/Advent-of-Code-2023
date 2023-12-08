use itertools::Itertools;
use std::collections::HashMap;
fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> i32 {
    let mut parts = input.split("\n\n");
    let lr_instructions = parts.next().unwrap().chars();

    let map: HashMap<[char; 3], ([char; 3], [char; 3])> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let chars = l
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect_vec();

            (
                (&chars[0..3]).try_into().unwrap(),
                (
                    (&chars[3..6]).try_into().unwrap(),
                    (&chars[6..9]).try_into().unwrap(),
                ),
            )
        })
        .collect();

    let mut current = map[&['A'; 3]];

    let mut steps = 0;
    loop {
        for c in lr_instructions.clone() {
            steps += 1;
            let next_id = match c {
                'L' => &current.0,
                'R' => &current.1,
                _ => panic!("Unknown instruction {}", c),
            };
            if next_id == &['Z'; 3] {
                return steps;
            }
            current = map[next_id];
        }
    }
}
