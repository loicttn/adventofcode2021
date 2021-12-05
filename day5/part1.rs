use std::str::FromStr;

#[derive(Debug)]
struct Point(usize, usize);

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
        Ok(Path { from: Point(from[0], from[1]), to: Point(to[0], to[1]) })
    }
}

struct Map {
    field: Vec<Vec<i32>>,
    size: usize,
}

impl Map {
    fn new(size: usize) -> Map {
        let mut v = Vec::new();

        for _ in 0..size {
            let mut line = Vec::new();
            for _ in 0..size {
                line.push(0 as i32);
            }
            v.push(line);
        }
        Map { field: v, size: size }
    }

    fn compute_path(&mut self, path: Path) {
        if path.from.0 == path.to.0 && path.from.1 != path.to.1 {
            let min = if path.from.1 > path.to.1 { path.to.1 } else { path.from.1 };
            let max = if path.from.1 > path.to.1 { path.from.1 } else { path.to.1 } + 1;
            for y in min..max {
                self.field[y][path.from.0] += 1;
            }
        }
        if path.from.1 == path.to.1 && path.from.0 != path.to.0 {
            let min = if path.from.0 > path.to.0 { path.to.0 } else { path.from.0 };
            let max = if path.from.0 > path.to.0 { path.from.0 } else { path.to.0 } + 1;
            for x in min..max {
                self.field[path.from.1][x] += 1;
            }
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
    let mut map = Map::new(1000);
    
    for path in pathes {
        match path {
            Ok(path) => map.compute_path(path),
            Err(_e) => ()
        }
    }

    println!("Solution is {}", map.compute_score())
}
