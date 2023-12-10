use std::fmt::{self, Display, Formatter};

fn main() {
    let input = include_str!("../../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    let maze = parse_maze(input);
    let mut path = Path {
        path: vec![maze.start],
        maze: &maze,
    };
    path.find_path();
    for (y, x) in &path.path {
        dbg!(&maze.board[*y][*x], y, x);
    }
    path.path.len()
}

fn parse_maze(input: &str) -> Maze {
    let mut rows = vec![];
    let mut start = (0, 0);
    for (row_id, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (col_id, c) in line.chars().enumerate() {
            if let Some(e) = Element::from_char(c) {
                row.push(e);
            } else {
                dbg!("Error parsing element", c);
                continue;
            }
            if c == 'S' {
                start = (row_id, col_id);
            }
        }
        rows.push(row);
    }
    Maze { board: rows, start }
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

impl Element {
    fn from_char(s: char) -> Option<Self> {
        match s {
            '|' => Some(Element::VerticalPipe),
            '-' => Some(Element::HorizontalPipe),
            'J' => Some(Element::NorthWestPipe),
            '7' => Some(Element::SouthWestPipe),
            'L' => Some(Element::NorthEastPipe),
            'F' => Some(Element::SouthEastPipe),
            '.' => Some(Element::Ground),
            'S' => Some(Element::Start),
            _ => None,
        }
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Element::VerticalPipe => "│",
            Element::HorizontalPipe => "─",
            Element::NorthWestPipe => "┘",
            Element::SouthWestPipe => "┐",
            Element::NorthEastPipe => "└",
            Element::SouthEastPipe => "┌",
            Element::Ground => "▓",
            Element::Start => "┼",
        };
        write!(f, "{}", s)
    }
}

type Board = Vec<Vec<Element>>;

struct Maze {
    board: Board,
    start: (usize, usize),
}

struct Path<'a> {
    path: Vec<(usize, usize)>,
    maze: &'a Maze,
}

impl<'a> Path<'a> {
    fn find_path(&mut self) {
        let mut curr = self.maze.start;
        let mut next = self.maze.start;
        while let Some(point) = next_point(&self.maze, next, curr) {
            if point == self.maze.start {
                break;
            }
            curr = next;
            next = point;
            self.path.push(point);
        }
        ()
    }
}

fn next_point(maze: &Maze, point: (usize, usize), prev: (usize, usize)) -> Option<(usize, usize)> {
    let dy = point.0 as i32 - prev.0 as i32;
    let dx = point.1 as i32 - prev.1 as i32;
    let element = &maze.board[point.0][point.1];
    match element {
        Element::VerticalPipe => {
            if dy == 0 {
                None
            } else {
                if dy > 0 {
                    Some((point.0 + 1, point.1))
                } else {
                    Some((point.0 - 1, point.1))
                }
            }
        }
        Element::HorizontalPipe => {
            if dy == 0 {
                if dx > 0 {
                    Some((point.0, point.1 + 1))
                } else {
                    Some((point.0, point.1 - 1))
                }
            } else {
                None
            }
        }
        Element::NorthWestPipe => {
            if dy == 0 && dx == 1 {
                Some((point.0 - 1, point.1))
            } else if dy == 1 && dx == 0 {
                Some((point.0, point.1 - 1))
            } else {
                None
            }
        }
        Element::SouthWestPipe => {
            if dy == 0 && dx == 1 {
                Some((point.0 + 1, point.1))
            } else if dy == -1 && dx == 0 {
                Some((point.0, point.1 - 1))
            } else {
                None
            }
        }
        Element::NorthEastPipe => {
            if dy == 0 && dx == -1 {
                Some((point.0 - 1, point.1))
            } else if dy == 1 && dx == 0 {
                Some((point.0, point.1 + 1))
            } else {
                None
            }
        }
        Element::SouthEastPipe => {
            if dy == 0 && dx == -1 {
                Some((point.0 + 1, point.1))
            } else if dy == -1 && dx == 0 {
                Some((point.0, point.1 + 1))
            } else {
                None
            }
        }
        Element::Ground => None,
        Element::Start => {
            if dy == 0 && dx == 0 {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .map(|(dy, dx)| (point.0 as i32 + dy, point.1 as i32 + dx))
                    .filter(|(y, x)| *y >= 0 && *x >= 0)
                    .map(|(y, x)| (y as usize, x as usize))
                    .find(|&(y, x)| next_point(maze, (y, x), point).is_some())
            } else {
                None
            }
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for e in row {
                write!(f, "{}", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
