const EPOCH: u32 = 80;

fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("Could not load input file");
    let mut numbers: Vec<u32> = content.split(",").map(|e| e.trim().parse().unwrap()).collect();

    for _e in 0..EPOCH {
        for i in 0..numbers.len() {
            if numbers[i] == 0 {
                numbers[i] = 6;
                numbers.push(8);
            } else {
                numbers[i] -= 1;
            }
        }
    }
    println!("Solution is {}", numbers.len());
}
