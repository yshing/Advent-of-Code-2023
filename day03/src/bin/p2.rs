use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input");
    dbg!(part2(input));
}

fn part2(input: &str) -> usize {
    let chars = input.as_bytes();
    let width = chars.iter().position(|&c| c == b'\n').unwrap();
    let height = chars.len() / (width + 1);

    let check_and_sum = |state: &mut State, col: usize, row: usize, value: usize, len: usize| {
        let surrounding = surroundings(chars, width, height, col - len, len, row);
        if surrounding.iter().any(|&c| match c {
            (_, b'\n') => false,
            (_, b'.') => false,
            (_, b'0'..=b'9') => false,
            (idx, b'*') => {
                state.gear_numbers.entry(idx).or_default().push(value);
                true
            }
            _ => true,
        }) {
            state.sum += value;
        }
        state.current_len = 0;
        state.current_value = 0;
    };
    let mut state = State::default();
    chars.iter().for_each(|&c| {
        let State {
            col,
            row,
            current_value,
            current_len,
            ..
        } = state;
        match c {
            b'0'..=b'9' => {
                state.current_value = state.current_value * 10 + (c - b'0') as usize;
                state.col += 1;
                state.current_len += 1;
            }
            b'\n' => {
                if state.current_len > 0 {
                    check_and_sum(&mut state, col, row, current_value, current_len);
                }
                state.col = 0;
                state.row += 1;
            }
            _ => {
                if state.current_len > 0 {
                    check_and_sum(&mut state, col, row, current_value, current_len);
                }
                state.col += 1;
            }
        }
    });
    state
        .gear_numbers
        .iter()
        .filter_map(|(_, v)| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum()
}

fn surroundings(
    board: &[u8],
    width: usize,
    height: usize,
    start_x: usize,
    len: usize,
    y: usize,
) -> Vec<(usize, u8)> {
    let mut chars = Vec::new();
    for &(dx, dy) in &[
        (-1, -1),
        (len as isize, -1),
        (-1, 0),
        (len as isize, 0),
        (-1, 1),
        (len as isize, 1),
    ] {
        let col = start_x as isize + dx;
        let row = y as isize + dy;
        if col >= 0 && col < width as isize && row >= 0 && row < height as isize {
            chars.push((
                (row * (width + 1) as isize + col) as usize,
                board[(row * (width + 1) as isize + col) as usize],
            ));
        }
    }
    if y as isize - 1 >= 0 {
        let start = (y - 1) * (width + 1) + start_x;
        for idx in start..start + len {
            chars.push((idx, board[idx]));
        }
    }
    if y + 1 < height {
        let start = (y + 1) * (width + 1) + start_x;
        for idx in start..start + len {
            chars.push((idx, board[idx]));
        }
    }
    chars
}

#[derive(Debug, Default)]
struct State {
    gear_numbers: HashMap<usize, Vec<usize>>,
    col: usize,
    row: usize,
    current_value: usize,
    current_len: usize,
    sum: usize,
}
