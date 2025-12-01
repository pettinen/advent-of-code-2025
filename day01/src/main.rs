#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    amount: i32,
}

fn parse() -> Vec<Rotation> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("failed to read stdin");
            let mut chars = line.chars();

            let direction = match chars.next() {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => {
                    panic!("invalid line: {line}")
                }
            };

            let mut amount = 0;
            for char in chars {
                amount *= 10;
                amount += match char.to_digit(10) {
                    Some(digit) => digit as i32,
                    None => {
                        panic!("invalid line: {line}");
                    }
                };
            }

            Rotation { direction, amount }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

fn solve(rotations: &[Rotation], part: PuzzlePart) {
    let mut dial = 50;
    let mut zeros = 0;
    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                if part == PuzzlePart::Two && rotation.amount >= dial {
                    zeros += (rotation.amount - dial) / 100 + if dial == 0 { 0 } else { 1 };
                }
                dial = (dial - rotation.amount).rem_euclid(100);
            }
            Direction::Right => {
                if part == PuzzlePart::Two && rotation.amount >= 100 - dial {
                    zeros += (dial + rotation.amount) / 100;
                }
                dial = (dial + rotation.amount).rem_euclid(100);
            }
        };
        if part == PuzzlePart::One && dial == 0 {
            zeros += 1;
        }
    }
    println!("{zeros}");
}

fn main() {
    let rotations = parse();
    solve(&rotations, PuzzlePart::One);
    solve(&rotations, PuzzlePart::Two);
}
