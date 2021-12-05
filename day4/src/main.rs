fn main() {
    let mut boards = vec![];
    let numbers: Vec<_> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut current = Board {
        numbers: vec![],
        mask: 0,
    };
    for line in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .skip(2)
    {
        if line.trim().is_empty() {
            boards.push(current.clone());
            current = Board {
                numbers: vec![],
                mask: 0,
            };
        }
        current
            .numbers
            .extend(line.split_whitespace().map(|s| s.parse::<u8>().unwrap()));
    }

    part1(numbers.clone(), &mut boards);
    part2(numbers, &mut boards);
}

fn part1(numbers: Vec<u8>, boards: &mut Vec<Board>) {
    println!("calculating winning board:");
    for n in numbers {
        for b in boards.iter_mut() {
            b.update(n);
            if b.win() {
                println!("boards : {:?} wins with last number {}", b, n);
                let s = b.unmarked().iter().map(|x| *x as u32).sum::<u32>();
                println!("unmarked sum of board is {}", s);
                println!("expected answer (product) is {}", s * (n as u32));
                return;
            }
        }
    }
}
fn part2(numbers: Vec<u8>, boards: &mut Vec<Board>) {
    println!("calculating losing board");
    let bl = boards.len();

    let mut boards_won: Vec<usize> = vec![];
    'outer: for n in &numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            if boards_won.contains(&i) {
                continue;
            }
            b.update(*n);
            if b.win() {
                boards_won.push(i);
            }
            if boards_won.len() == bl - 1 {
                break 'outer;
            }
        }
    }
    let mut last_board: Board = boards
        .iter()
        .enumerate()
        .filter(|(i, b)| !boards_won.contains(i))
        .next()
        .unwrap()
        .1
        .clone();

    last_board.mask = 0;

    for n in numbers {
        last_board.update(n);
        if last_board.win() {
            println!("boards : {:?} wins last with last number {}", last_board, n);
            let s = last_board.unmarked().iter().map(|x| *x as u32).sum::<u32>();
            println!("unmarked sum of board is {}", s);
            println!("expected answer (product) is {}", s * (n as u32));
            return;
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    numbers: Vec<u8>,
    mask: u32,
}

impl Board {
    fn update(&mut self, number: u8) {
        for (i, &n) in self.numbers.iter().enumerate() {
            if n == number {
                self.mask |= 1 << i;
            }
        }
    }

    fn win(&self) -> bool {
        let mut rows = [0u32; 5];
        for r in 0..=4 {
            for i in 0..=4 {
                rows[r] += 1 << (5 * r + i);
            }
        }

        let mut cols = [0u32; 5];
        for c in 0..=4 {
            for i in 0..=4 {
                cols[c] += 1 << (c + 5 * i);
            }
        }
        return rows.iter().chain(cols.iter()).any(|m| m & self.mask == *m);
    }

    fn unmarked(&self) -> Vec<u8> {
        let mut acc = vec![];
        for (i, _) in self.numbers.iter().enumerate() {
            if self.mask & (1 << i) == 0 {
                acc.push(self.numbers[i] as u8);
            }
        }
        return acc;
    }
}
