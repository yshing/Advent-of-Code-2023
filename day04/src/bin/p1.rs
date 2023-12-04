use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    input.lines().map(|x| Card::from(x)).map(|card| {
        let count_winning = card
            .numbers
            .iter()
            .filter(|&x| card.winning_numbers.contains(x))
            .count();
        2_usize.pow((count_winning as u32).saturating_sub(1))
    }).sum()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
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
            .map(|x| x.trim().parse::<_>().unwrap())
            .sorted()
            .collect::<Vec<_>>();

        let numbers = numbers_iter
            .map(|x| x.trim().parse::<_>().unwrap())
            .sorted()
            .collect::<Vec<_>>();

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_card_parsing() {
        let input = "Card   1: 98 16 95 90 53 33 43  7 46 45 | 85 15 78 57 34 10 46 90 33 13  8 54  4 37 25 63 55 41  7 82 69 16 30 76  2";
        let card = Card::from(input);
        assert_eq!(card.id, 1);
        assert_eq!(
            card.winning_numbers,
            vec![98, 16, 95, 90, 53, 33, 43, 7, 46, 45]
        );
        assert_eq!(
            card.numbers,
            vec![
                85, 15, 78, 57, 34, 10, 46, 90, 33, 13, 8, 54, 4, 37, 25, 63, 55, 41, 7, 82, 69,
                16, 30, 76, 2
            ]
        );
    }
    #[test]
    fn test_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_2() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        assert_eq!(part1(input), 2);
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(part1(input), 2);
        let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        assert_eq!(part1(input), 1);
        let input = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        assert_eq!(part1(input), 0);
        let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 0);
    }
}
