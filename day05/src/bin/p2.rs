use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let mut seeds = parse_seeds(parts.next().unwrap());
    while let Some(input) = parts.next() {
        let mapping = Mapping::from_str(input);
        seeds = mapping.map_ranges(seeds);
    }
    *seeds.iter().map(|(seed, _)| seed).min().unwrap()
}

#[derive(Debug)]
struct Mapping<'a> {
    source: &'a str,
    dest: &'a str,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    dest_start: usize,
    source_start: usize,
    size: usize,
}

fn parse_seeds(input: &str) -> Vec<(usize, usize)> {
    let numbers = input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    numbers
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect()
}

impl<'a> Mapping<'a> {
    fn from_str(input: &'a str) -> Self {
        let mut lines = input.lines();
        let header = lines.next().unwrap();
        let title = header.split_whitespace().nth(0).unwrap();
        let mut title_split = title.split("-to-");
        let source = title_split.next().unwrap();
        let dest = title_split.next().unwrap();
        Self {
            source,
            dest,
            ranges: lines
                .map(|line| Range::from_str(line))
                .sorted_by(|a, b| a.source_start.cmp(&b.source_start))
                .collect(),
        }
    }

    fn map_ranges(&self, seed_ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for seed_range in seed_ranges.iter() {
            let mut remanding_range = Option::Some(*seed_range);
            self.ranges
                .iter()
                .filter(|range| range.is_overlap(*seed_range))
                .for_each(|range| {
                    let (ranges, remainder) = range.map_range(remanding_range.unwrap());
                    result.extend(ranges);
                    remanding_range = remainder;
                });
            if let Some(remainder) = remanding_range {
                if remainder.1 > 0 {
                    result.push(remainder);
                }
            }
        }
        result
    }
}

impl Range {
    fn from_str(input: &str) -> Self {
        let mut iter = input.split_whitespace();
        let dest_start = iter.next().unwrap().parse().unwrap();
        let source_start = iter.next().unwrap().parse().unwrap();
        let size = iter.next().unwrap().parse().unwrap();
        Self {
            dest_start,
            source_start,
            size,
        }
    }

    // Bad example of implementing this function following the spec, should use some math instead
    fn map_range(&self, range: (usize, usize)) -> (Vec<(usize, usize)>, Option<(usize, usize)>) {
        let (start, len) = range;
        let source_start = self.source_start;
        let source_end = self.source_start + self.size;
        let end = start + len - 1;

        let overlap_start = std::cmp::max(start, source_start);
        let overlap_end = std::cmp::min(end, source_end);

        if overlap_start < overlap_end {
            let mut result = vec![];
            if start < self.source_start {
                result.push((start, self.source_start - start));
            }
            result.push((
                self.dest_start + overlap_start - self.source_start,
                overlap_end - overlap_start,
            ));
            if end >= source_end {
                result.push((
                    self.dest_start + source_end - self.source_start,
                    end - source_end + 1,
                ));
            }
            return (result, None);
        }
        return (vec![], Some((start, len)));
    }

    fn is_overlap(&self, range: (usize, usize)) -> bool {
        let (start, len) = range;
        let end = start + len - 1;
        if end < self.source_start {
            false
        } else if start >= self.source_start + self.size {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_map_range() {
        let range = Range {
            dest_start: 10,
            source_start: 10,
            size: 10,
        };
        assert_eq!(range.map_range((0, 10)), (vec![(0, 10)], None));
        assert_eq!(range.map_range((0, 5)), (vec![(0, 5)], None));
        assert_eq!(range.map_range((0, 15)), (vec![(0, 10), (10, 5)], None));
        assert_eq!(
            range.map_range((0, 25)),
            (vec![(0, 10), (10, 10)], Some((20, 5)))
        );
    }

    #[test]
    fn test_range_map_range_2() {
        let range = Range {
            dest_start: 10,
            source_start: 10,
            size: 2,
        };
        assert_eq!(range.map_range((11, 1)), (vec![(11, 1)], None));
        assert_eq!(range.map_range((9, 2)), (vec![(9, 1), (10, 1)], None));
        assert_eq!(
            range.map_range((9, 4)),
            (vec![(9, 1), (10, 2)], Some((12, 1)))
        );
    }

    #[test]
    fn test_mapping_map_ranges() {
        let mapping = Mapping {
            source: "source",
            dest: "dest",
            ranges: vec![
                Range {
                    dest_start: 10,
                    source_start: 10,
                    size: 10,
                },
                Range {
                    dest_start: 20,
                    source_start: 20,
                    size: 10,
                },
            ],
        };
        assert_eq!(mapping.map_ranges(vec![(0, 10)]), vec![(0, 10)]);
        assert_eq!(mapping.map_ranges(vec![(0, 15)]), vec![(0, 10), (10, 5)]);
        assert_eq!(
            mapping.map_ranges(vec![(0, 25)]),
            vec![(0, 10), (10, 10), (20, 5)]
        );
        assert_eq!(
            mapping.map_ranges(vec![(0, 35)]),
            vec![(0, 10), (10, 10), (20, 10), (30, 5)]
        );
    }

    #[test]
    fn part2_sample() {
        let input = include_str!("../../sample");
        assert_eq!(part2(input), 46);
    }
}
