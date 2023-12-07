use std::str::Lines;

fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> usize {
    let Game { time, dist } = Game::from_str(input);
    count_wins(time, dist)
}

fn count_wins(time: usize, dist: usize) -> usize {
    (1..time).fold(0, |n, s| if (time - s) * s >= dist { n + 1 } else { n })
}

#[derive(Debug)]
struct Game {
    time: usize,
    dist: usize,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let parse_next = |l: &mut Lines<'_>| {
            l.next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .join("")
                .parse()
                .unwrap()
        };
        let time = parse_next(&mut lines);
        let dist = parse_next(&mut lines);
        Self { time, dist }
    }
}
