struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, r: &Range) -> bool {
        for i in r.start..=r.end {
            if !self.contains_num(i) {
                return false;
            }
        }
        true
    }

    fn contains_num(&self, i: usize) -> bool {
        i >= self.start && i <= self.end
    }

    fn overlaps(&self, r: &Range) -> bool {
        for i in r.start..=r.end {
            if self.contains_num(i) {
                return true;
            }
        }
        false
    }
}

pub fn pt1(input: &str) -> Result<usize, String> {
    let res = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .filter(|line| {
            let ranges: Vec<usize> = line
                .split(&['-', ','][..])
                .flat_map(|e| e.parse())
                .collect();

            match ranges[..] {
                [s1, e1, s2, e2] => {
                    let r1 = Range { start: s1, end: e1 };
                    let r2 = Range { start: s2, end: e2 };

                    r1.contains(&r2) || r2.contains(&r1)
                }
                _ => false,
            }
        })
        .collect::<Vec<&&str>>()
        .len();

    match res {
        ans if ans > 0 => Ok(ans),
        _ => Err("nope".to_string()),
    }
}

pub fn pt2(input: &str) -> Result<usize, String> {
    let lines = input.lines().collect::<Vec<&str>>();

    match part_2(lines) {
        ans if ans > 0 => Ok(ans),
        _ => Err("nope".to_string()),
    }
}

fn part_2(lines: Vec<&str>) -> usize {
    lines
        .iter()
        .filter(|line| {
            let ranges: Vec<usize> = line
                .split(&['-', ','][..])
                .flat_map(|e| e.parse())
                .collect();

            match ranges[..] {
                [s1, e1, s2, e2] => {
                    let r1 = Range { start: s1, end: e1 };
                    let r2 = Range { start: s2, end: e2 };

                    r1.overlaps(&r2) || r2.overlaps(&r1)
                }
                _ => false,
            }
        })
        .collect::<Vec<&&str>>()
        .len()
}
