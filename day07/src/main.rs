#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Position {
    Beam,
    Space,
    Splitter,
}

fn parse() -> Vec<Vec<Position>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|char| match char {
                    'S' => Position::Beam,
                    '.' => Position::Space,
                    '^' => Position::Splitter,
                    _ => {
                        panic!("invalid input");
                    }
                })
                .collect()
        })
        .collect()
}

fn part1(input: &mut [Vec<Position>]) {
    let mut splits = 0;

    for y in 1..input.len() {
        for x in 0..input[y].len() {
            if input[y - 1][x] == Position::Beam {
                if input[y][x] == Position::Space {
                    input[y][x] = Position::Beam;
                } else if input[y][x] == Position::Splitter {
                    splits += 1;
                    input[y][x - 1] = Position::Beam;
                    input[y][x + 1] = Position::Beam;
                }
            }
        }
    }

    println!("{splits}");
}

fn part2(input: &[Vec<Position>]) {
    let mut paths: Vec<Vec<u64>> = Vec::with_capacity(input.len());

    for y in 0..input.len() {
        let mut new_paths = Vec::with_capacity(input[y].len());

        for x in 0..input[y].len() {
            if y == 0 {
                new_paths.push(if input[y][x] == Position::Beam { 1 } else { 0 });
                continue;
            }
            if input[y][x] != Position::Beam {
                new_paths.push(0);
                continue;
            }

            let splitter_left = x > 0 && input[y][x - 1] == Position::Splitter;
            let splitter_right = input[y].get(x + 1) == Some(&Position::Splitter);

            new_paths.push(
                paths[y - 1][x]
                    + if splitter_left {
                        paths[y - 1]
                            .get(x.wrapping_sub(1))
                            .copied()
                            .unwrap_or_default()
                    } else {
                        0
                    }
                    + if splitter_right {
                        paths[y - 1].get(x + 1).copied().unwrap_or_default()
                    } else {
                        0
                    },
            );
        }
        paths.push(new_paths);
    }

    println!("{}", paths.last().unwrap().iter().sum::<u64>());
}

fn main() {
    let mut input = parse();
    part1(&mut input);
    part2(&input);
}
