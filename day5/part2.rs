use std::str::FromStr;

#[derive(Debug)]
struct Point(i64, i64);

#[derive(Debug)]
struct Path {
    from: Point,
    to: Point,
}

#[derive(Debug)]
#[non_exhaustive]
enum Error {
    ParseError
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<String> = s.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>();
        if points.len() != 2 || points[0].len() == 0 || points[1].len() == 0 {
            return Err(Error::ParseError);
        }
        let from: Vec<usize> = points[0].split(",").map(|e| e.parse().unwrap()).collect();
        let to: Vec<usize> = points[1].split(",").map(|e| e.parse().unwrap()).collect();
        Ok(Path { from: Point(from[0] as i64, from[1] as i64), to: Point(to[0] as i64, to[1] as i64) })
    }
}

struct Map {
    field: Vec<Vec<i64>>,
    size: usize,
}

impl Map {
    fn new(size: usize) -> Map {
        let mut v = Vec::new();

        for _ in 0..size {
            let mut line = Vec::new();
            for _ in 0..size {
                line.push(0 as i64);
            }
            v.push(line);
        }
        Map { field: v, size: size }
    }

    fn compute_path(&mut self, path: Path) {
        let mut x_diff: i64 = path.to.0 - path.from.0 ;
        let mut y_diff: i64 = path.to.1 - path.from.1;
    
        if x_diff != 0 {
            x_diff /= x_diff.abs();
        }
        if y_diff != 0 {
            y_diff /= y_diff.abs();
        }
        let mut x = path.from.0;
        let mut y = path.from.1;
        while x != path.to.0 + x_diff || y != path.to.1 + y_diff {
            self.field[y as usize][x as usize] += 1;
            x += x_diff;
            y += y_diff;
        }
    }

    fn compute_score(&self) -> u32 {
        let mut score = 0;

        for y in 0..self.size {
            for x in 0..self.size {
                if self.field[y][x] > 1 {
                    score += 1;
                }
            }
        }
        score
    }
}

fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("Could not read input file");
    let pathes: Vec<Result<Path, _>> = content.split("\n").map(|e| Path::from_str(e)).collect();
    let mut map = Map::new(999);
    
    for path in pathes {
        match path {
            Ok(path) => map.compute_path(path),
            Err(_e) => ()
        }
    }

    println!("Solution is {}", map.compute_score())
}
