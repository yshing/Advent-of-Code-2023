fn main() {
    let input = include_str!("../../input");
    dbg!(part_2(&input));
}

fn part_2(arr: &str) -> usize {
    arr.lines()
        .map(find_number)
        .sum::<usize>()
}

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_number(input: &str) -> usize {
    find_digit(input, false) * 10 + find_digit(input, true)
}

fn find_digit(input: &str, reverse: bool) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        let idx = if reverse { input.len() - i - 1 } else { i };
        if chars[idx].is_ascii_digit() {
            return chars[idx].to_digit(10).unwrap() as usize;
        }
        let word_digit = NUMBERS.iter().enumerate().find(|(_, &word)| {
            if reverse {
                if word.len() + idx > input.len() {
                    return false;
                }
                if word == &input[idx..idx + word.len()] {
                    return true;
                }
            } else {
                if word.len() + idx > input.len() {
                    return false;
                }
                if word == &input[idx..idx + word.len()] {
                    return true;
                }
            }
            false
        });
        if let Some((index, _)) = word_digit {
            return index + 1 as usize;
        }
    }
    panic!("No number found in string {}", &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_digit_return_correct_value_from_the_end() {
        assert_eq!(find_digit("pxreightwo7", true), 7);
        assert_eq!(find_digit("pxreightwo", true), 2);
    }

    #[test]
    fn find_digit_return_correct_value_from_the_begining() {
        assert_eq!(find_digit("eightwo7", false), 8);
    }
}
