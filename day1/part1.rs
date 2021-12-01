use std::fs;

fn main() {
     let content: String = fs::read_to_string("input.txt").expect("Could not load input file");
     let mut a: i64 = -1;
     let mut total: i64 = 0;

     for x in content.split("\n") {
        let i: i64 = match x.parse() {
            Ok(n) => n,
            Err(_e) => break
        };
        if i > a && a != -1 {
            total += 1
        }
        a = i;
     }
    println!("Solution is {}", total);
}
