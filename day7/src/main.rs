fn main() {
    part1();
    // part2();
}
fn part1() {
    let mut v: Vec<_> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split(",")
        .map(|s| str::parse::<usize>(s.trim()).unwrap())
        .collect();
    println!("len v = {}", v.len());
    println!("average v = {}", v.iter().sum::<usize>() as f64 /v.len() as f64);
    v.sort();
    let mid1= v.len()/2;
    let mid2 = (v.len() +1) / 2;
    let median: i64 = ((v[mid1] + v[mid2])/2) as i64;
    println!("median = {}", median);

    let fuel_conso:i64 = v.iter().map(|&x| i64::abs(x as i64 - median as i64)).sum();

    println!("fuel conso v1= {}", fuel_conso);

    let average = (v.iter().sum::<usize>() / v.len()) as i64;
    let fuel_conso:i64 = v.iter().map(|&x| i64::abs(x as i64 - average as i64)*(i64::abs(x as i64 - average)+1)/2).sum();
    let average = (v.iter().sum::<usize>() / v.len()) as i64 +1 ;
    let fuel_conso2:i64 = v.iter().map(|&x| i64::abs(x as i64 - average as i64)*(i64::abs(x as i64 - average)+1)/2).sum();
    println!("fuel conso v2= {}", std::cmp::min(fuel_conso, fuel_conso2));
} 
