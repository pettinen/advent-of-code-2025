use std::io::Read as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Op {
    Add,
    Mul,
}

fn parse_part1(s: &str) -> ([Vec<u64>; 4], Vec<Op>) {
    let mut values = <[Vec<u64>; 4]>::default();
    let mut ops = Vec::new();

    for (i, line) in s.lines().enumerate() {
        for s in line.split_whitespace() {
            if i < 4 {
                values[i].push(s.parse().unwrap());
            } else {
                ops.push(match s {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => {
                        panic!("invalid input");
                    }
                });
            }
        }
    }
    (values, ops)
}

fn part1(values: &[Vec<u64>; 4], ops: &[Op]) {
    let mut total = 0;
    for i in 0..values[0].len() {
        total += match ops[i] {
            Op::Add => values[0][i] + values[1][i] + values[2][i] + values[3][i],
            Op::Mul => values[0][i] * values[1][i] * values[2][i] * values[3][i],
        };
    }
    println!("{total}");
}

fn part2(s: &str) {
    let lines: [_; 5] = s
        .lines()
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut total = 0;
    let mut numbers = Vec::new();
    let mut op = None;
    for i in 0..=lines[0].len() {
        if i == lines[0].len() || lines.iter().all(|line| line[i] == b' ') {
            let mut acc = match op {
                Some(Op::Add) => 0,
                Some(Op::Mul) => 1,
                None => {
                    panic!("missing op");
                }
            };
            for number in &numbers {
                match op {
                    Some(Op::Add) => {
                        acc += number;
                    }
                    Some(Op::Mul) => {
                        acc *= number;
                    }
                    None => unreachable!(),
                }
            }
            total += acc;
            op = None;
            numbers.clear();
        } else {
            match lines[4][i] {
                b'+' => {
                    op = Some(Op::Add);
                }
                b'*' => {
                    op = Some(Op::Mul);
                }
                _ => {}
            }

            let mut pos = 1;
            let mut number = 0u64;
            for j in (0..4).rev() {
                if lines[j][i].is_ascii_digit() {
                    number += pos * u64::from(lines[j][i] - 0x30);
                    pos *= 10;
                }
            }
            numbers.push(number);
        }
    }

    println!("{total}");
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (values, ops) = parse_part1(&input);
    part1(&values, &ops);
    part2(&input);
}
