use itertools::{self, Itertools};
fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| Hand::from_str(l))
        .sorted()
        .enumerate()
        .map(|(rank, hand)| ((rank + 1) * hand.bid))
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pattern {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
struct Card(char);

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    bid: usize,
    cards: [Card; 5],
    weights: [usize; 5],
    pattern: Pattern,
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        let mut parts = s.split_whitespace();
        let cards: [Card; 5] = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| Card(c))
            .collect_vec()
            .try_into()
            .unwrap();
        let bid = parts.next().unwrap().parse().unwrap();

        let weights: [usize; 5] = cards
            .iter()
            .map(|c| c.weight())
            .collect_vec()
            .try_into()
            .unwrap();
        let pattern = Hand::get_pattern(&weights);

        Hand {
            cards,
            weights,
            pattern,
            bid,
        }
    }

    fn get_pattern(weights: &[usize; 5]) -> Pattern {
        let weights_count = weights
            .iter()
            .dedup_with_count()
            .map(|(count, &weight)| (weight, count))
            .sorted()
            .collect_vec();
        return match weights_count.len() {
            1 => Pattern::FiveOfAKind,
            2 => match weights_count[0] {
                (1, _) => Pattern::FiveOfAKind,
                (_, count) => {
                    if count == 4 || count == 1 {
                        Pattern::FourOfAKind
                    } else {
                        Pattern::FullHouse
                    }
                }
            },
            3 => match weights_count[0] {
                (1, _) => match std::cmp::max(weights_count[1].1, weights_count[2].1) {
                    1 => Pattern::FourOfAKind,
                    2 => {
                        if weights_count[1].1 + weights_count[2].1 == 4 {
                            Pattern::FullHouse
                        } else {
                            Pattern::FourOfAKind
                        }
                    }
                    3 => Pattern::FourOfAKind,
                    n => panic!("Unexpected max count of non J cards: {}", n),
                },
                (_, count) => {
                    if [count, weights_count[1].1, weights_count[2].1]
                        .iter()
                        .max()
                        .unwrap()
                        == &3
                    {
                        Pattern::ThreeOfAKind
                    } else {
                        Pattern::TwoPairs
                    }
                }
            },
            4 => match weights_count[0] {
                (1, count_j) => match count_j {
                    1 => Pattern::TwoPairs,
                    2 => Pattern::ThreeOfAKind,
                    _ => panic!(
                        "Unexpected count_j: {}, given 4 different cards in hand",
                        count_j
                    ),
                },
                (_, _) => Pattern::OnePair,
            },
            5 => match weights_count[0] {
                (1, _) => Pattern::OnePair,
                (_, _) => Pattern::HighCard,
            },
            n => panic!("Unexpected number of different cards in hand: {}", n),
        };
    }
}

impl Card {
    fn weight(&self) -> usize {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '2'..='9' => self.0.to_digit(10).unwrap() as usize,
            _ => 0,
        }
    }
}

impl std::cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight().partial_cmp(&other.weight())
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.pattern.cmp(&other.pattern) {
            std::cmp::Ordering::Equal => self
                .weights
                .iter()
                .zip(other.weights.iter())
                .find_map(|(a, b)| {
                    let cmp = a.cmp(b);
                    if cmp != std::cmp::Ordering::Equal {
                        Some(cmp)
                    } else {
                        None
                    }
                })
                .or(Some(std::cmp::Ordering::Equal)),
            other => Some(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = include_str!("../../sample");
        assert_eq!(part2(sample), 5905);
    }

    #[test]
    fn test_hand_compare() {
        let hand1 = Hand::from_str("KTJJT 684");
        let hand2 = Hand::from_str("KK677 28");
        assert_eq!(hand2 > hand1, true);
    }
}
