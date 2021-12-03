use std::fs::*;

const N: usize = 1000;
const M: usize = 12;

fn part_1(values: &[Vec<u8>]) -> u32 {
    let half_n = N / 2;
    let mut gamma = 0;
    
    for j in 0..M {
        let mut curr = 0;
        for i in 0..N {
            if values[i][j] == 1 {
                curr += 1;
                if curr == half_n {
                    gamma += 1 << (M - 1 - j);
                    break;
                }
            }
        }
    }

    gamma * ((1 << M) - gamma - 1)
}

fn main() {
    let input = read_to_string("inputs/day3/input")
        .expect("Cannot read the file");
    
    let values: Vec<Vec<u8>> = input.lines()
        .map(|line| {
            line.chars()
                .map(|i| if i == '0' { 0 } else { 1 })
                .collect()
        })
        .collect();

    println!("{}", part_1(&values));
}
