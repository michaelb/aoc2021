fn main() {
    part1();
    part2();
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
fn part2() {
    let i = std::fs::read_to_string("input.txt").unwrap();
    let inputs = i
        .lines()
        .map(|s| s.split('|').next().unwrap())
        .map(|s| {
            s.split_whitespace()
                .map(|s2| sanitize(s2))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    let inputs_str: Vec<Vec<&str>> = inputs
        .iter()
        .map(|V| V.iter().map(|S| &S as &str).collect::<Vec<&str>>())
        .collect();

    // println!("input {:?}", inputs);
    // .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)

    let orders: Vec<_> = inputs_str.iter().map(|v| guess(v.clone())).collect();
    let guesser = |(ordered, output): (&[&str], &str)| {
        ordered
            .iter()
            // .inspect(|v| println!("inspect {:?} in output? ({:?}", v, output))
            .position(|&s| s == output)
            .unwrap()
    };

    let outputs = i
        .lines()
        .map(|s| s.split('|').nth(1).unwrap())
        .map(|s| s.split_whitespace() .map(|s2| sanitize(s2))
             .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let output_str: Vec<Vec<&str>> = outputs
        .iter()
        .map(|V| V.iter().map(|S| &S as &str).collect::<Vec<&str>>())
        .collect();

    let output_as_num: Vec<Vec<usize>> = orders
        .iter()
        .zip(outputs.iter())
        .map(|(ord, out)| out.iter().map(|s| guesser((ord, s))).collect())
        .collect();


    let output_as_num: Vec<_> = output_as_num.iter().map(|v| v[0] * 1000 + v[1] * 100 + v[2] * 10 + v[3]).collect();
    println!("output num {:?}", output_as_num);

    println!("output product {:?}",output_as_num.iter().sum::<usize>());

}

pub fn sanitize(input: &str) -> String {
    let mut v: Vec<char> = input.trim().chars().collect();
    v.sort();
    return v
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
}

fn guess(input: Vec<&str>) -> Vec<&str> {
    // println!("input: {:?}", input);
    let segment6 = input
        .iter()
        .filter(|s| s.len() == 6)
        .map(|s| sanitize(s))
        .collect::<Vec<_>>(); // 6,0 or 9
    let segment5 = input
        .iter()
        .filter(|s| s.len() == 5)
        .map(|s| sanitize(s))
        .collect::<Vec<_>>(); // 5, 2 or 3

    let pos1 = (0..input.len()).find(|&i| input[i].len() == 2).unwrap();
    let pos4 = (0..input.len()).find(|&i| input[i].len() == 4).unwrap();
    let pos7 = (0..input.len()).find(|&i| input[i].len() == 3).unwrap();
    let pos8 = (0..input.len()).find(|&i| input[i].len() == 7).unwrap();

    let pos069 = (0..input.len())
        .filter(|&i| input[i].len() == 6)
        .collect::<Vec<usize>>();
    let pos235 = (0..input.len())
        .filter(|&i| input[i].len() == 5)
        .collect::<Vec<usize>>();
    // println!("inputs {:?}", input);
    // println!("069: {:?}", pos069);
    // println!("235: {:?}", pos235);
    // println!("positions parsed");

    let perms = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for p1 in perms {
        for p2 in perms {
            let total_perm = [
                pos069[p1[0]],
                pos1,
                pos235[p2[0]],
                pos235[p2[1]],
                pos4,
                pos235[p2[2]],
                pos069[p1[1]],
                pos7,
                pos8,
                pos069[p1[2]],
            ];
            let mut ordered_input = ["z"; 10];
            for i in 0..10 {
                // println!("{:?}", input[total_perm[i]]);
                ordered_input[i] = input[total_perm[i]];
            }

            if possible(&ordered_input) {
                println!("order ipnut : {:?}", ordered_input);
                return ordered_input.to_vec();
            }
        }
    }
    println!("Impossible!");
    vec![]
}

fn possible(i: &[&str]) -> bool {
    commons(i[0], i[1]) == 2 // i0 is 0 or 9
        && commons(i[0], i[4]) == 3 // i0 is 0
        && commons(i[6], i[4]) == 3 // i6 is 6
        && commons(i[3], i[7]) == 3 //i3 is 3
        && commons(i[5], i[6]) == 5 //i5 is 5
}

fn commons(i1: &str, i2: &str) -> usize {
    i1.chars().filter(|c| i2.contains(&c.to_string())).count()
}

mod test {
    use super::*;
    #[test]
    fn test_sanitize() {
        let input = "abcdegf";
        println!("{}", sanitize(input));
        assert_eq!(sanitize(input), "abcdefg");
    }

    #[test]
    fn test_commons() {
        assert_eq!(commons("abcdef", "bcg"), 2);
    }
}
