use std::fs;

fn convert(bits: &[u32]) -> u32 {
    let mut result: u32 = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit;
    });
    result
}

fn find_var(values: &Vec<String>, bit: char, revbit: char) -> u32 {
    let mut buff: Vec<String> = values.to_vec();
    let mut i = 0;
    while true {
        if buff.len() == 1 {
             break;
        }
        let mut bits = (0, 0);
        for e in &buff {
            if e.len() != 12 {
                break;
            }
            let bit = e.chars().nth(i).unwrap();
            bits.0 += if bit == '1' { 0 } else { 1 };
            bits.1 += if bit == '0' { 0 } else { 1 };
        }
        let selected_bit = if bits.0 > bits.1 { bit } else {revbit };
        buff = buff.into_iter().filter(|e| e.len() == 12 && e.chars().nth(i).unwrap() == selected_bit).collect::<Vec<_>>();
        i += 1;

    }
    let o2 = buff[0].clone();
    convert(&o2.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>())
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot load input file");
    let values: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();

    println!("Solution is {}", find_var(&values, '0', '1') * find_var(&values, '1', '0'))
}
