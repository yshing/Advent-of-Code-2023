use itertools::Itertools;
fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(predict_prev)
        .sum()
}

fn predict_prev(input: Vec<i64>) -> i64 {
    let mut numbers = input.clone();
    let mut stack = vec![numbers.first().unwrap().clone()];
    loop {
        let next_numbers = numbers.windows(2).map(|w| w[1] - w[0]).collect_vec();

        if next_numbers.iter().all_equal() {
            return stack.iter().rfold(next_numbers[0], |acc, curr| curr - acc);
        }

        stack.push(next_numbers.first().unwrap().clone());
        numbers = next_numbers;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_prev() {
        let input = vec![10,13,16,21,30,45];
        assert_eq!(predict_prev(input), 5);
    }
}
