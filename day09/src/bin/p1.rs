use itertools::Itertools;
fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(predict_next)
        .sum()
}

fn predict_next(input: Vec<i64>) -> i64 {
    let mut numbers = input.clone();
    let mut stack = vec![numbers.last().unwrap().clone()];
    loop {
        let next_numbers = numbers.windows(2).map(|w| w[1] - w[0]).collect_vec();

        if next_numbers.iter().all_equal() {
            return stack.iter().rfold(next_numbers[0], |acc, curr| acc + curr);
        }

        stack.push(next_numbers.last().unwrap().clone());
        numbers = next_numbers;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_1() {
        let input = vec![0, 3, 6];
        assert_eq!(predict_next(input), 9);
        let input = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next(input), 28);
    }
}
