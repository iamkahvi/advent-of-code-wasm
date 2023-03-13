use std::{collections::HashSet, slice::Windows};

pub fn pt1(input: &str) -> Result<usize, String> {
    let lines = input.lines().collect::<Vec<&str>>();

    solve_line(lines.first().unwrap(), 4)
}

pub fn pt2(input: &str) -> Result<usize, String> {
    let lines = input.lines().collect::<Vec<&str>>();

    solve_line(lines.first().unwrap(), 14)
}

fn solve_line(line: &str, window_size: usize) -> Result<usize, String> {
    let chars = line.chars().collect::<Vec<char>>();
    let windows: Windows<char> = chars.windows(window_size);

    'outer: for (i, w) in windows.enumerate() {
        let mut h = HashSet::<&char>::new();
        for (j, c) in w.iter().enumerate() {
            if h.contains(c) {
                continue 'outer;
            }
            h.insert(c);
            if j == (w.len() - 1) {
                return Ok(i + window_size);
            }
        }
    }

    Err("nope".to_string())
}
