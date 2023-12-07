use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let mut seeds = parse_seeds(parts.next().unwrap());
    while let Some(input) = parts.next() {
        let mapping = Mapping::from_str(input);
        seeds.iter_mut().for_each(|seed| *seed = mapping.map(*seed));
    }
    *seeds.iter().min().unwrap()
}


#[derive(Debug)]
struct Mapping <'a>{
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

fn parse_seeds(input: &str) -> Vec<usize> {
    input.split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap()).collect_vec()
}

impl <'a> Mapping <'a> {
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
            ranges: lines.map(|line| Range::from_str(line)).collect(),
        }
    }

    fn map(&self, num: usize) -> usize {
        if let Some(range) = self.ranges.iter().find(|range| range.is_in_range(num)) {
            range.map(num)
        } else {
            num
        }
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

    fn is_in_range(&self, num: usize) -> bool {
        num >= self.source_start && num < self.source_start + self.size
    }

    fn map(&self, num: usize) -> usize {
        assert!(self.is_in_range(num));
        num - self.source_start + self.dest_start
    }
}


