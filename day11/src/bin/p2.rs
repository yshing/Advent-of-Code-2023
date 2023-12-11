use itertools::Itertools;
fn main() {
    let input = include_str!("../../input");
    println!("part1: {}", part2(input, 1000000));
}

fn part2(input: &str, space: usize) -> usize {
    let mut lines = input.lines();
    let width = lines.by_ref().next().unwrap().len();
    let height = lines.by_ref().count() + 1;
    let mut x_set = vec![false; width];
    let mut y_set = vec![false; height];

    // parse galaxy
    let mut stars = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                x_set[x] = true;
                y_set[y] = true;
                stars.push((x, y));
            }
        }
    }

    // expand x
    let mut stars_expanded = stars.clone();
    for x in 0..width {
        if x_set[x] {
            continue;
        }
        for id in 0..stars.len() {
            if stars[id].0 > x {
                stars_expanded[id].0 += space - 1;
            }
        }
    }

    for y in 0..height {
        if y_set[y] {
            continue;
        }
        for id in 0..stars.len() {
            if stars[id].1 > y {
                stars_expanded[id].1 += space - 1;
            }
        }
    }

    (0..stars.len())
        .combinations(2)
        .map(|ids| {
            let a = stars_expanded[ids[0]];
            let b = stars_expanded[ids[1]];
            let dx = (a.0 as isize - b.0 as isize).abs();
            let dy = (a.1 as isize - b.1 as isize).abs();
            (dy + dx) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("../../sample");
        assert_eq!(part2(input, 10), 1030);
        assert_eq!(part2(input, 100), 8410);
    }
}
