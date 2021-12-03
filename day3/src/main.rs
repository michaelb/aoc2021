fn main() {
    let mut acc = vec![0; 12];
    let mut len = 0;
    for number in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
    {
        for (i, c) in number.chars().enumerate() {
            match c.to_digit(10).unwrap() {
                1 => acc[i] += 1,
                _ => (),
            }
        }
        len += 1;
    }
    let gamma: Vec<u32> = acc
        .iter()
        .map(|x| match x * 2 > len {
            true => 1,
            false => 0,
        })
        .collect();
    let epsilon: Vec<u32> = gamma.iter().map(|x| 1 - x).collect();

    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);
    let gamma = usize::from_str_radix(
        &gamma
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap();
    let epsilon = usize::from_str_radix(
        &epsilon
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap();

    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);

    println!("gamm * epsilon: {}", gamma * epsilon);
    part2();
}

fn most_common_bit_at(values: &Vec<Vec<u32>>, position: u32) -> u32 {
    match values
        .iter()
        .map(|v| v[position as usize] as usize)
        .sum::<usize>()
        * 2
        >= values.len()
    {
        true => 1,
        false => 0,
    }
}

fn least_common_bit_at(values: &Vec<Vec<u32>>, position: u32) -> u32 {
    match values
        .iter()
        .map(|v| v[position as usize] as usize)
        .sum::<usize>()
        * 2
        < values.len()
    {
        true => 1,
        false => 0,
    }
}

fn part2() {
    let mut values: Vec<Vec<u32>> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let mut values2 = values.clone();

    let mut ox_rating = 0;
    for i in 0..12 {
        let bit = most_common_bit_at(&values, i);
        values = values
            .clone()
            .into_iter()
            .filter(|v| v[i as usize] == bit)
            .collect();
        println!("most common bit: {:?}", bit);
        println!("vec lenght {}", values.len());
        if values.len() <= 1 {
            ox_rating = u32::from_str_radix(
                &values[0]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                2,
            )
            .unwrap();
            break;
        }
    }

    println!("yyyyyy");
    let mut co2_rating = 0;
    for i in 0..12 {
        let bit = least_common_bit_at(&values2, i);
        values2 = values2
            .clone()
            .into_iter()
            .filter(|v| v[i as usize] == bit)
            .collect();

        println!("least common bit: {:?}", bit);
        println!("vec lenght {}", values2.len());
        if values2.len() == 2 {
            println!("vec here: {:?}",values2);
        }
        if values2.len() <= 1 {
            co2_rating = u32::from_str_radix(
                &values2[0]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                2,
            )
            .unwrap();
            break;
        }
    }

    println!(
        "ox: {}, co2: {}, product: {}",
        ox_rating,
        co2_rating,
        ox_rating * co2_rating
    );
}
