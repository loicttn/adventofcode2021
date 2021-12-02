use std::fs;

fn main() {
    let content: String = fs::read_to_string("input.txt").expect("Could not load input file");
    let mut x = 0;
    let mut y = 0;

    for instruction in content.split("\n") {
        let sp = instruction.split(" ").collect::<Vec<&str>>();
        if sp.len() != 2 {
            break;
        }
        let step = match sp[1].parse() {
            Ok(v) => v,
            Err(_e) => 0,
        };
        if sp[0] == "forward" {
            x += step;
        }
        if sp[0] == "down" {
            y += step;
        }
        if sp[0] == "up" {
            y -= step;
        }
    }
    println!("Solution is {}", x * y);
}
