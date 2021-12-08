fn main() {
    part1();
    // part2();
}
fn part1() {
    println!(
        "number of 1,4,7,8: {}",
        std::fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|s| s.split('|').skip(1).next().unwrap())
            .map(|s| s.split_whitespace())
            .flatten()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count()
    );



}

pub fn sanitize(input: &str) -> String {
    let mut v : Vec<char> = input.trim().chars().collect();
    v.sort();
    return v.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");
}

fn guess(input: Vec<&str>) -> Box<dyn Fn(&str) -> i32> {
    let segment6 = input.iter().filter(|s| s.len() == 6).map(|s| sanitize(s)).collect::<Vec<_>>(); // 6,0 or 9
    let segment5 = input.iter().filter(|s| s.len() == 5).map(|s| sanitize(s)).collect::<Vec<_>>(); // 5, 2 or 3

    let pos1 = (0..(input.len()-1)).filter(|&i| input[i].len() == 2).next().unwrap();
    let pos4 = (0..(input.len()-1)).filter(|&i| input[i].len() == 4).next().unwrap();
    let pos7 = (0..(input.len()-1)).filter(|&i| input[i].len() == 3).next().unwrap();
    let pos8 = (0..(input.len()-1)).filter(|&i| input[i].len() == 7).next().unwrap();

    let pos069 = (0..(input.len()-1)).filter(|&i| input[i].len() == 5).collect::<Vec<usize>>();
    let pos235 = (0..(input.len()-1)).filter(|&i| input[i].len() == 6).collect::<Vec<usize>>();


    let perms = [[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
    for p1 in perms {
        for p2 in perms {
            let total_perm = [pos069[0], pos1, pos235[0], pos235[1], pos4, pos235[2], pos069[1], pos7, pos8, pos069[2]];
            let mut ordered = input.clone();
            ordered.sort_by_key(|s| total_perm[input.iter().position(|s2| s2 == s).unwrap()]);
            println!("{:?}", ordered);


        }
    }

    Box::new(|s| 3)

}

fn possible(orderer_input: Vec<&str>) {

}



fn commons(i1: &str, i2:&str) -> usize {
    i1.chars().filter(|c| i2.contains(&c.to_string())).count()
}



    

mod test {
    use super::*;
    #[test]
    fn test_sanitize(){
        let input = "abcdegf";
        println!("{}", sanitize(input));
        assert_eq!(sanitize(input),"abcdefg");
    }
}
