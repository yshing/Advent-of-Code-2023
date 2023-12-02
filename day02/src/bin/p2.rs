use std::cmp::max;
fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(trim_line)
        .map(|line| {
            // Get min number of cubes for each game
            line.split(";")
                .map(count_cubes)
                .reduce(|a, b| (max(a.0, b.0), max(a.1, b.1), max(a.2, b.2)))
                .unwrap()
        })
        // to power
        .map(|(r, g, b)| r * g * b)
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
