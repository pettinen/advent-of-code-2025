use std::collections::HashSet;

fn parse() -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if let Some((a, b)) = line.split_once('-') {
            ranges.push((a.parse().unwrap(), b.parse().unwrap()));
        } else if !line.is_empty() {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

fn part1(ranges: &[(u64, u64)], ids: &[u64]) {
    let mut fresh = 0;

    for id in ids {
        for (a, b) in ranges {
            if a <= id && id <= b {
                fresh += 1;
                break;
            }
        }
    }
    println!("{fresh}");
}

fn part2(ranges: &[(u64, u64)]) {
    let mut breakpoints = Vec::new();
    for (a, b) in ranges {
        for x in [a, b] {
            breakpoints.push(*x);
            breakpoints.push(x + 1);
        }
    }

    breakpoints.sort_unstable();
    let mut included_breakpoints = HashSet::new();
    for breakpoint in &breakpoints {
        for (a, b) in ranges {
            if a <= breakpoint && breakpoint <= b {
                included_breakpoints.insert(*breakpoint);
                break;
            }
        }
    }

    let mut current_start = None;
    let mut total = 0;
    for breakpoint in breakpoints {
        if included_breakpoints.contains(&breakpoint) {
            if current_start.is_none() {
                current_start = Some(breakpoint);
            }
        } else if let Some(start) = current_start {
            total += breakpoint - start;
            current_start = None;
        }
    }
    println!("{total}");
}

fn main() {
    let (ranges, ids) = parse();
    part1(&ranges, &ids);
    part2(&ranges);
}
