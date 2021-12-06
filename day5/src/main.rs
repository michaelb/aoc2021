use std::cmp::max;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl From<&str> for Point {
    fn from(s: &str) ->Self {
        let v: Vec<&str> = s.split(",").map(|s| s.trim()).collect();
        Point{
            x: v[0].parse().unwrap(),
            y: v[1].parse().unwrap(),
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn covers(&self, diags:bool) -> Vec<Point> {
        if self.is_vertical() {
            return (self.start.y..=self.end.y).map(|y| Point {x:self.start.x, y}).collect();
        }
        if self.is_horizontal() {
            
            return (self.start.x..=self.end.x).map(|x| Point {x, y:self.start.y}).collect();
        }

        // diagonal at 45%
        if diags {
            let slope = match self.start.y < self.end.y {
                true => 1,
                false => -1
            };

            return (self.start.x..=self.end.x).map(|x| Point {x, y:(self.start.y as isize +slope* (x as isize - self.start.x as isize)) as usize}).collect();
        }

        vec![]
    }

    fn size(&self) -> usize {
        max(max(self.start.x, self.end.x), max(self.start.y, self.end.y))
    }
}


type Grid = Vec<Vec<usize>>;

fn create_grid(dimensions: (usize, usize))  -> Grid{
    let (x,y) =dimensions;
    let mut g:Grid = vec![];
    for i in 0..=y {
        let tmp_vec = vec![0; x];
        g.push(tmp_vec);
    }
    g
}

fn update_grid(g:&mut Grid, l:&Line, diags:bool) {
    println!("line {:?} covers {:?}",l, l.covers(true));
    for p in l.covers(diags) {
        let Point{x,y} = p;
        g[x][y] += 1;
    }
}

fn count(g:&Grid) -> usize{
    g.into_iter().flatten().filter(|&&v| v >= 2 ).count()
}

fn display(g:&Grid) {
    for v in g {
        println!("{:?}", v);
    }
}


fn main() {
    //  for line in std::fs::read_to_string("input.txt")
    //     .expect("Couldn't read input.txt")
    //     .lines()
    // {
    let lines:Vec<Line> =std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .lines()
        .map(|l| {
            let v: Vec<&str> = l.split("->").take(2).collect();
            let v: Vec<Point> = v.iter().map(|p| Point::from(*p)).collect();
            let (p1, p2) = (v[0], v[1]);
            if p1.x > p2.x ||  p1.x == p2.x && p1.y > p2.y {

                Line {
                    start: p2,
                    end: p1,
                }
            } else {
                Line {start:p1, end:p2}
            }

        }).collect();

    let dims = lines.iter().map(|l| l.size()).max().unwrap() + 1 ;
    // println!("lines: {:?}", lines);
    println!("dims {}",dims);
    println!("conut liens : {:?}", lines.len());

    let mut g = create_grid((dims,dims));

    for l in &lines {
        update_grid(&mut g, &l, false);
    }
    // display(&g);
    println!("count of dangerous zones: {}", count(&mut g));

    let mut g = create_grid((dims,dims));

    for l in lines {
        update_grid(&mut g, &l, true);
    }

    println!("count of dangerous zones with diags: {}", count(&mut g));



}
