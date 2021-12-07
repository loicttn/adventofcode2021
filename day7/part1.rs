fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("Could not load input file");
    let numbers: Vec<i32> = content.split(",").map(|e| e.trim().parse().unwrap()).collect();

    let mut min = 0;
    for i in 0..*numbers.iter().max().unwrap() {
        let mut tmp_min = 0;
        for x in &numbers {
            tmp_min += (x - i).abs();
        }
        if tmp_min < min || min == 0 {
            min = tmp_min;
        }
    }
    println!("Solution is {}", min);
}
