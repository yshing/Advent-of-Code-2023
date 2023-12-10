use std::{str::FromStr, path::Display};

fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize{

}


fn parse_maze(input: &str) -> Vec<Vec<Element>> {
    input.lines().map(|line| {
        line.split("").filter_map(|s|Element::from_str(s).ok()).collect()
    }).collect_vec()
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum Element {
    VerticalPipe = b'|',
    HorizontalPipe = b'-',
    NorthWestPipe = b'J',
    SouthWestPipe = b'7',
    NorthEastPipe = b'L',
    SouthEastPipe = b'F',
    Ground = b'.',
    Start = b'S',
}

impl FromStr for Element {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Element::VerticalPipe),
            "-" => Ok(Element::HorizontalPipe),
            "J" => Ok(Element::NorthWestPipe),
            "7" => Ok(Element::SouthWestPipe),
            "L" => Ok(Element::NorthEastPipe),
            "F" => Ok(Element::SouthEastPipe),
            "." => Ok(Element::Ground),
            "S" => Ok(Element::Start),
            _ => Err(()),
        }
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Element::VerticalPipe => "|",
            Element::HorizontalPipe => "-",
            Element::NorthWestPipe => "J",
            Element::SouthWestPipe => "7",
            Element::NorthEastPipe => "L",
            Element::SouthEastPipe => "F",
            Element::Ground => ".",
            Element::Start => "S",
        };
        write!(f, "{}", s)
    }
}