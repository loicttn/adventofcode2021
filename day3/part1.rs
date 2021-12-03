use std::fs;

fn convert(bits: &[u32]) -> u32 {
    let mut result: u32 = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit;
    });
    result
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot load input file");
    let values = content.split("\n");
    let mut buffer = [(0, 0); 12];

    for val in values {
        if val.len() != 12 {
            break;
        }
        for i in 0..12 {
            let bit: char = val.chars().nth(i).unwrap();
            buffer[i].0 += if bit == '0' {1} else {0};
            buffer[i].1 += if bit == '1' {1} else {0};
        }
    }
    let mut gamma_rate = Vec::from([0;12]);
    for i in 0..buffer.len() {
        if buffer[i].0 > buffer[i].1 {
            gamma_rate[i] = 0;
        } else {
            gamma_rate[i] = 1;
        }
    }
    let epsilon_rate = &gamma_rate.iter().map(|e| if *e == 1 {0} else {1}).collect::<Vec<_>>();

    println!("Solution is {}", convert(&gamma_rate) * convert(&epsilon_rate));
}
