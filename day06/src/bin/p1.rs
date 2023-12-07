use std::str::Lines;

fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    let game = Game::from_str(input);
    game.time
        .iter()
        .zip(game.dist.iter())
        .map(|(&t, &d)| count_wins(t, d))
        .product()
}

fn count_wins(time: usize, dist: usize) -> usize {
    (1..time).fold(0, |n, s| if (time - s) * s >= dist { n + 1 } else { n })
}

#[derive(Debug)]
struct Game {
    time: Vec<usize>,
    dist: Vec<usize>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let parse_next = |l: &mut Lines<'_>| {
            l.next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect()
        };
        let time = parse_next(&mut lines);
        let dist = parse_next(&mut lines);
        Self { time, dist }
    }
}
