use std::collections::HashMap;

fn unique_len(s: &str) -> usize {
    let mut m: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        let val = m.entry(c).or_insert(0);
        *val += 1;
    }
    let mut len = 0;
    for (_, value) in m {
        len += if value == 1 {1} else {0};
    }
    len
}

fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("Could not load input file");
    let signals: Vec<Vec<&str>> = content.split("\n").filter(|e| e.len() > 1).map(|e| e.split(" | ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()).collect();

    let mut ttl = 0;
    for s in &signals {
        for o in s {
            let z = unique_len(o);
            if z == 2 || z == 4 || z == 3 || z == 7 {
                ttl += 1;
            }
        }
    }
    println!("Solution is {}", ttl);
}
