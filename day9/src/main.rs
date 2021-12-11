fn main() {
    part1();
    part2();
}
fn part1() {
    let mut v: Vec<Vec<_>> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let (height, width) = (v.len(), v[0].len());

    let mut acc = 0;
    for i in 0..height {
        for j in 0..width {
            if lowest(i, j, (height as isize, width as isize), &v) {
                acc += v[i][j] + 1;
            }
        }
    }

    println!("{}", acc);
}

fn neighbors(i: isize, j: isize, dims: (isize, isize)) -> impl Iterator<Item = (usize, usize)> {
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
        .into_iter()
        .filter(move |(i, j)| i >= &0 && i < &dims.0 && j >= &0 && j < &dims.1)
        .map(|(i, j)| ((i as usize), (j as usize)))
}

fn lowest(i: usize, j: usize, dims: (isize, isize), v: &[Vec<usize>]) -> bool {
    neighbors(i as isize, j as isize, dims).all(|(i2, j2)| v[i][j] < v[i2][j2])
}

fn go_down(i: usize, j: usize, dims: (isize, isize), v: &[Vec<usize>]) -> (usize, usize) {
    if lowest(i, j, dims, v) {
        return (i, j);
    }
    let (i_lower, j_lower) = neighbors(i as isize, j as isize, dims)
        .min_by_key(|&(i2, j2)| v[i2][j2])
        .unwrap();
    return go_down(i_lower, j_lower, dims, v);
}

fn part2() {
    let mut v: Vec<Vec<_>> = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let (height, width) = (v.len(), v[0].len());

    let mut basins: std::collections::HashMap<(usize,usize), Vec<_>> = std::collections::HashMap::new();

    for i in 0..height {
        for j in 0..width {
            if v[i][j] == 9 {
                continue;
            }
            let lowest= go_down(i, j, (height as isize, width as isize), &v);
            let mut basin = basins.get(&lowest).unwrap_or(&vec![]).to_vec();
            // println!("basin whose to add a coord {:?}", basin);
            basin.push((i,j));
            basins.insert(lowest,basin);
        }
    }

    let mut basins = basins.into_iter().collect::<Vec<_>>();
    basins.sort_by_key(|(_,val)| -(val.len() as isize));
    // println!("basins : {:?}", &basins[..3]);

    let sol = basins[0].1.len() * basins[1].1.len() * basins[2].1.len();
    println!("sol : {:?}", sol);
}
