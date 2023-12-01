fn main() {
    let input = include_str!("../../input");
    dbg!(part_1(&input));
}

fn part_1(arr: &str) -> usize {
    arr.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|chars| {
            chars
                .iter()
                .filter(|c| c.is_ascii_digit())
                .copied()
                .collect::<Vec<char>>()
        })
        .map(|chars| {
            chars.first().unwrap().to_digit(10).unwrap() * 10
                + chars.last().unwrap().to_digit(10).unwrap()
        })
        .sum::<u32>() as usize
}