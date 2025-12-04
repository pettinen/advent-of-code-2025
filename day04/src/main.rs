#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Position {
    Empty,
    Filled,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Map(Vec<Vec<Position>>);

impl Map {
    fn less_than_4_adjacent(&self, y: usize, x: usize) -> bool {
        if self.is_filled(y, x) == 0 {
            return false;
        }
        self.is_filled(y - 1, x - 1)
            + self.is_filled(y - 1, x)
            + self.is_filled(y - 1, x + 1)
            + self.is_filled(y, x - 1)
            + self.is_filled(y, x + 1)
            + self.is_filled(y + 1, x - 1)
            + self.is_filled(y + 1, x)
            + self.is_filled(y + 1, x + 1)
            < 4
    }

    fn clear(&mut self, y: usize, x: usize) {
        self.0[y - 1][x - 1] = Position::Empty;
    }

    fn count_filled(&self) -> usize {
        let mut total = 0;
        for row in &self.0 {
            for col in row {
                if col == &Position::Filled {
                    total += 1;
                }
            }
        }
        total
    }

    fn is_filled(&self, y: usize, x: usize) -> u8 {
        if y == 0 || x == 0 {
            return 0;
        }
        if let Some(row) = self.0.get(y - 1)
            && row.get(x - 1) == Some(&Position::Filled)
        {
            return 1;
        }
        0
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

fn parse() -> Map {
    let rows = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("could not read stdin");
            line.chars()
                .map(|char| match char {
                    '.' => Position::Empty,
                    '@' => Position::Filled,
                    _ => {
                        panic!("invalid input");
                    }
                })
                .collect()
        })
        .collect();
    Map(rows)
}

fn part1(map: &Map) {
    let mut total = 0;

    for y in 1..=map.height() {
        for x in 1..=map.width() {
            if map.less_than_4_adjacent(y, x) {
                total += 1;
            }
        }
    }

    println!("{total}");
}

fn clear_removable(map: &mut Map) -> bool {
    let mut cleared_any = false;

    for y in 1..=map.height() {
        for x in 1..=map.width() {
            if map.less_than_4_adjacent(y, x) {
                map.clear(y, x);
                cleared_any = true;
            }
        }
    }

    cleared_any
}

fn part2(map: &mut Map) {
    let initial_count = map.count_filled();
    while clear_removable(map) {}
    let count_removed = initial_count - map.count_filled();
    println!("{count_removed}");
}

fn main() {
    let mut map = parse();
    part1(&map);
    part2(&mut map);
}
