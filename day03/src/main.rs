fn parse() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("could not read stdin");
            line.chars()
                .map(|char| char.to_digit(10).expect("invalid input") as u8)
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<u8>]) {
    let mut total = 0u32;

    for row in input {
        let mut first_digit_indices = [None; 9];
        let mut last_digit_indices = [None; 9];

        for (i, value) in row.iter().enumerate() {
            let array_idx = *value as usize - 1;
            if first_digit_indices[array_idx].is_none() {
                first_digit_indices[array_idx] = Some(i);
            }
            last_digit_indices[array_idx] = Some(i);
        }

        let mut first_digit_idx = None;
        let mut second_digit_idx = None;
        for digit_idx in first_digit_indices.into_iter().rev() {
            if let Some(digit_idx) = digit_idx
                && digit_idx != row.len() - 1
            {
                first_digit_idx = Some(digit_idx);
                break;
            }
        }
        let first_digit_idx = first_digit_idx.unwrap();

        for digit_idx in last_digit_indices.into_iter().rev() {
            if let Some(digit_idx) = digit_idx
                && digit_idx > first_digit_idx
            {
                second_digit_idx = Some(digit_idx);
                break;
            }
        }
        let second_digit_idx = second_digit_idx.unwrap();

        total += 10 * u32::from(row[first_digit_idx]) + u32::from(row[second_digit_idx]);
    }

    println!("{total}");
}

fn to_u64(digits_rev: &[u8], except: Option<usize>) -> u64 {
    let mut total = 0;
    let mut i = 0;
    for &digit in digits_rev.iter().rev() {
        if except == Some(11 - i) {
            i += 1;
            continue;
        }
        total *= 10;
        total += u64::from(digit);
        i += 1;
    }
    total
}

fn part2(input: &[Vec<u8>]) {
    let mut total = 0;

    for row in input {
        let mut digits_rev = Vec::with_capacity(12);
        digits_rev.extend(row[row.len() - 12..].iter().rev());

        for &digit in row[..row.len() - 12].iter().rev() {
            if digit >= digits_rev[11] {
                let mut max_value_if_removed = 0;
                let mut max_value_if_removed_idx = 0;
                for i in (0..12).rev() {
                    let value_if_removed = to_u64(&digits_rev, Some(i));
                    if value_if_removed > max_value_if_removed {
                        max_value_if_removed = value_if_removed;
                        max_value_if_removed_idx = i;
                    }
                }

                digits_rev.remove(max_value_if_removed_idx);
                digits_rev.push(digit);
            }
        }
        total += to_u64(&digits_rev, None);
    }

    println!("{total}");
}

fn main() {
    let input = parse();
    part1(&input);
    part2(&input);
}
