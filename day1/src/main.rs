fn main() {
    part1();
    part2();
}
fn part1() {
    let v: Vec<_> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .map(|s| str::parse::<usize>(s).unwrap())
        .collect();
    let c = v
        .iter()
        .zip(v.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();
    println!("Increases count: {}", c);
}

fn part2() {
    let v: Vec<_> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .map(|s| str::parse::<usize>(s).unwrap())
        .collect();
    let s: Vec<_> = v
        .iter()
        .zip(v.iter().skip(1))
        .zip(v.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();
    let c = s
        .iter()
        .zip(s.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();
    println!(" Increases count (sums): {}", c);
}
