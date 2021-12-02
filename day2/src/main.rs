fn main() {
    part1();
    part2();
}
fn part1() {
    let mut x = 0;
    let mut depth = 0;
    for command in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
    {
        let (direction, value) = match &command.split_whitespace().collect::<Vec<&str>>()[..] {
            &[a, b, ..] => (a, b.parse::<i64>().unwrap()),
            _ => unreachable!(),
        };
        match (direction, value) {
            ("forward", value) => x = x + value,
            ("down", value) => depth = depth + value,
            ("up", value) => depth = depth - value,
            _ => unreachable!(),
        }
    }

    println!("final depth {}, final forward pos: {}", depth, x);
    println!("product:  {}", depth * x);
}

fn part2() {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
    {
        let (direction, value) = match &command.split_whitespace().collect::<Vec<&str>>()[..] {
            &[a, b, ..] => (a, b.parse::<i64>().unwrap()),
            _ => unreachable!(),
        };
        match (direction, value) {
            ("forward", value) => {
                x = x + value;
                depth += value * aim;
            }
            ("down", value) => aim += value,
            ("up", value) => aim -= value,
            _ => unreachable!(),
        }
    }

    println!("final depth {}, final forward pos: {}", depth, x);
    println!("product:  {}", depth * x);
}
