fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    let max_cubes = (12, 13, 14);
    input
        .lines()
        .map(trim_line)
        .enumerate()
        .filter_map(|(i, line)| {
            line.split(";")
                .map(count_cubes)
                .all(|cubes| cubes <= max_cubes)
                .then(|| i + 1)
        })
        .sum()
}

fn trim_line(line: &str) -> &str {
    if let Some(i) = line.chars().position(|c| c == ':') {
        &line[i + 1..]
    } else {
        panic!("Invalid line: {}", line)
    }
}

fn count_cubes(input: &str) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    input
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.split(" "))
        .for_each(|mut split| {
            let n = split.next().unwrap().parse::<usize>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => red += n,
                "green" => green += n,
                "blue" => blue += n,
                _ => panic!("Invalid color: {}", color),
            }
        });

    (red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_cubes() {
        assert_eq!(count_cubes("1 red, 2 green, 3 blue"), (1, 2, 3));
        assert_eq!(count_cubes("15 red, 2 green, 3 blue"), (15, 2, 3));
    }
}
