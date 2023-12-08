use itertools::Itertools;
use std::collections::HashMap;
fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> i64 {
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

    map.keys()
        .filter(|k| k[2] == 'A')
        .map(|start| count_steps(&map, lr_instructions.clone(), start))
        .fold(1, num_integer::lcm)
}

fn count_steps(
    map: &HashMap<[char; 3], ([char; 3], [char; 3])>,
    instructions: std::str::Chars<'_>,
    start: &[char; 3],
) -> i64 {
    let mut current = map[start];
    let mut steps = 0;
    loop {
        for c in instructions.clone() {
            steps += 1;
            let next_id = match c {
                'L' => &current.0,
                'R' => &current.1,
                _ => panic!("Unknown instruction {}", c),
            };
            if next_id[2] == 'Z' {
                return steps;
            }
            current = map[next_id];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("../../sample2");
        assert!(part2(input) == 6)
    }
}
