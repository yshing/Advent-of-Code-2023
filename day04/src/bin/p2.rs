use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> usize {
    let cards = input.lines().map(|x| Card::from(x)).collect::<Vec<_>>();
    let cards_count = cards.len();
    let mut cards_nums = vec![1; cards_count];
    let winning_counts = cards.iter().map(|x| x.winning_count).collect::<Vec<_>>();
    winning_counts.iter().enumerate().for_each(|(id, &x)| {
        (id + 1..=id + x).for_each(|next_id| cards_nums[next_id] += cards_nums[id])
    });

    cards_nums.iter().sum()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_count: usize,
}

impl Card {
    fn from(input: &str) -> Self {
        let mut iter = input.split(":");
        let id_str = iter
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .trim();
        let id = usize::from_str_radix(id_str, 10).unwrap();

        let mut numbers_iter = iter.next().unwrap().split_whitespace();

        let winning_numbers = numbers_iter
            .by_ref()
            .take_while(|&x| x != "|")
            .map(|x| x.trim().parse::<usize>().unwrap())
            .sorted()
            .collect::<Vec<_>>();

        let numbers = numbers_iter
            .map(|x| x.trim().parse::<_>().unwrap())
            .sorted()
            .collect::<Vec<_>>();

        let winning_count = numbers
            .iter()
            .filter(|i| winning_numbers.contains(i))
            .count();
        Self { id, winning_count }
    }
}
