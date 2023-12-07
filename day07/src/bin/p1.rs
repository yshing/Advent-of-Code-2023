use itertools::{self, Itertools};
fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
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
        let sorted_weights = weights.iter().sorted().collect_vec();

        if sorted_weights.iter().all_equal() {
            return Pattern::FiveOfAKind;
        }
        if sorted_weights[0..4].iter().all_equal() || sorted_weights[1..5].iter().all_equal() {
            return Pattern::FourOfAKind;
        }
        if (sorted_weights[0..3].iter().all_equal() && sorted_weights[3..5].iter().all_equal())
            || (sorted_weights[0..2].iter().all_equal() && sorted_weights[2..5].iter().all_equal())
        {
            return Pattern::FullHouse;
        }
        if (0..3).any(|i| sorted_weights[i..i + 3].iter().all_equal()) {
            return Pattern::ThreeOfAKind;
        }
        match sorted_weights.iter().sorted().dedup().count() {
            3 => return Pattern::TwoPairs,
            4 => return Pattern::OnePair,
            _ => return Pattern::HighCard,
        }
    }
}

impl Card {
    fn weight(&self) -> usize {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
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
        assert_eq!(part1(sample), 6440);
    }

    #[test]
    fn test_hand_compare() {
        let hand1 = Hand::from_str("KTJJT 684");
        let hand2 = Hand::from_str("KK677 28");
        assert_eq!(hand2 > hand1, true);
    }
}
