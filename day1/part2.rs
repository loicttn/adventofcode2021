use std::fs;

fn main() {
     let content: String = fs::read_to_string("input.txt").expect("Could not load input file");
     let mut total: i32 = 0;
     let list: Vec<i32> = Vec::from(content.split("\n").flat_map(|s| s.parse()).collect::<Vec<_>>());
     for x in 1..(list.len()-2) {
        let a: i32 = list[x-1] + list[x] + list[x+1];
        let b: i32 = list[x] + list[x+1] + list[x+2];
        if b > a {
            total += 1;
        }
     }
    println!("Solution is {}", total);
}
