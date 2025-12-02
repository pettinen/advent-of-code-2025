use std::io::Read as _;

fn parse() -> Vec<(u64, u64)> {
    let mut string = String::new();
    std::io::stdin()
        .read_to_string(&mut string)
        .expect("failed to get input");
    string
        .trim_end()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').expect("invalid input");
            (
                start.parse().expect("invalid input"),
                end.parse().expect("invalid input"),
            )
        })
        .collect()
}

fn part1(ranges: &[(u64, u64)]) {
    let mut invalid_ids_sum = 0;

    for range in ranges {
        for id in range.0..=range.1 {
            let digits = id.to_string();
            let digits_len = digits.len();

            if !digits_len.is_multiple_of(2) {
                continue;
            }

            if digits[..digits_len / 2] == digits[digits_len / 2..] {
                invalid_ids_sum += id;
            }
        }
    }

    println!("{invalid_ids_sum}");
}

fn part2(ranges: &[(u64, u64)]) {
    let mut invalid_ids_sum = 0;

    for range in ranges {
        for id in range.0..=range.1 {
            let digits = id.to_string();
            let digits_len = digits.len();

            for slice_len in 1..=digits_len / 2 {
                if !digits_len.is_multiple_of(slice_len) {
                    continue;
                }
                if (1..digits_len / slice_len).all(|slice_idx| {
                    digits[..slice_len]
                        == digits[slice_idx * slice_len..(slice_idx + 1) * slice_len]
                }) {
                    invalid_ids_sum += id;
                    break;
                }
            }
        }
    }

    println!("{invalid_ids_sum}");
}

fn main() {
    let ranges = parse();
    part1(&ranges);
    part2(&ranges);
}
