fn main() {
    part1();
    // part2();
}
fn part1() {
    let v: Vec<_> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split(",")
        .filter(|s| str::parse::<usize>(s).is_ok())
        .map(|s| str::parse::<usize>(s).unwrap())
        .collect();

    let mut bucket = [0usize; 9]; //represent 'age' of the fish
    
    for i in 0..=8 {
        bucket[i] = v.iter().filter(|&&x| x == i).count();
    }
    println!("initial bucket: {:?}", bucket);

    for _ in 0..80 {
        bucket = next(&bucket);
    }

    println!("after 80 days fish count: {:?}", bucket.iter().sum::<usize>());

    for _ in 80..256 {
        bucket = next(&bucket);
    }
    println!("after 256 days fish count: {:?}", bucket.iter().sum::<usize>());


 }


fn next(b: &[usize]) -> [usize;9]{
    let mut tmp = [0usize; 9];
    for i in 1..=8 {
        tmp[i-1] = b[i];
    }
    tmp[6] += b[0];
    tmp[8] = b[0];

    return tmp;
}
