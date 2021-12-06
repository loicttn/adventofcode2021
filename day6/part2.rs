const EPOCH: u32 = 256;

fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("Could not load input file");
    let numbers: Vec<u64> = content.split(",").map(|e| e.trim().parse().unwrap()).collect();

    let mut buff = Vec::from([0; 9]);

    for n in numbers {
        buff[n as usize] += 1;
    }
    for _e in 0..EPOCH {
        buff.rotate_left(1);
        buff[6] += buff[8];
    }

    println!("Solution is {}", buff.into_iter().sum::<u64>());
}
