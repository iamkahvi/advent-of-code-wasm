use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    first: Vec<Item>,
    second: Vec<Item>,
}

#[derive(Debug)]

pub struct Er(&'static str);

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Item(u8);

impl std::convert::TryFrom<u8> for Item {
    type Error = Er;

    fn try_from(value: u8) -> Result<Self, Er> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(Er("not a valid item")),
        }
    }
}

impl Item {
    fn score(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

pub fn pt1(input: &str) -> Result<usize, Er> {
    let lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    if lines.len() <= 1 {
        return Err(Er("nope"));
    };

    let sacks = parse_lines(&lines)?;

    let matches: Vec<Item> = sacks
        .iter()
        .filter_map(|s| {
            for c in &s.first {
                if s.second.contains(c) {
                    return Some(*c);
                }
            }
            None
        })
        .collect();

    let ans = matches.iter().map(|i| dbg!(i.score())).sum();

    Ok(ans)
}

pub fn pt2(input: &str) -> Result<usize, Er> {
    let lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    if lines.len() <= 1 {
        return Err(Er("nope"));
    };

    let ans = part_2(&lines)?;

    Ok(ans)
}

fn parse_lines(lines: &[&str]) -> Result<Vec<Rucksack>, Er> {
    let r = lines
        .iter()
        .filter_map(|l| {
            let len = l.len();

            if len % 2 != 0 {
                return None;
            }

            let (first, second) = l.split_at(len / 2);

            Some(Rucksack {
                first: first.chars().map(|c| Item(c as u8)).collect(),
                second: second.chars().map(|c| Item(c as u8)).collect(),
            })
        })
        .collect();

    Ok(r)
}

// ["dsfd", "asdf", "asdfdf"]
// go through each line
// create a hashset
// intersect 3 hashsets

fn part_2(lines: &[&str]) -> Result<usize, Er> {
    let hs: Vec<HashSet<Item>> = lines
        .iter()
        .map(|l| l.chars().map(|c| Item(c as u8)).collect())
        .collect();

    let matches: Vec<Item> = hs
        .iter()
        .tuples()
        .flat_map(|(a, b, c)| {
            let common: Vec<Item> = a
                .iter()
                .copied()
                .filter(|e| b.contains(e) && c.contains(e))
                .collect();

            if common.len() != 1 {
                dbg!(a);
                dbg!("wtf");
            }

            common
        })
        .collect();

    let ans = matches.iter().map(|i| dbg!(i.score())).sum();

    Ok(ans)
}
