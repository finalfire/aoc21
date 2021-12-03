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

enum Mode { OXYGEN, CO2, }

fn foo(candidates: &mut [bool], i: usize, mode: Mode, values: &[Vec<u8>]) {
    // if we have only one number, we shillin
    if candidates.iter().filter(|c| **c).count() == 1 {
        return;
    }
    
    // check most/less common from candidates with true value
    let (mut n0, mut n1) = (0, 0);
    for j in 0..candidates.len() {
        if candidates[j] {
            match values[j][i] {
                0 => n0 += 1,
                1 => n1 += 1,
                _ => {}
            };
        }
    }

    // bit is our target bit, mode defines the order
    let bit = match mode {
        Mode::OXYGEN => if n0 > n1 { 0 } else { 1 },
        Mode::CO2 => if n0 > n1 { 1 } else { 0 }
    };
    
    // update candidates
    for j in 0..candidates.len() {
        if candidates[j] {
            if values[j][i] != bit {
                candidates[j] = false;
            }
        }
    }

    foo(candidates, i+1, mode, values);
}

fn part_2(values: &[Vec<u8>]) -> u32 {
    let vec2num = |v: &[u8]| -> u32 {
        v.iter().enumerate().fold(0, |acc, (i, x)| acc + ((*x as u32) << M - i - 1))
    };

    let mut candidates = vec![true; N];
    foo(&mut candidates, 0, Mode::OXYGEN, values);
    let oxygen_index = candidates.iter().enumerate()
        .find(|x| *x.1 == true)
        .map(|x| x.0)
        .unwrap();
    
    candidates = vec![true; N];
    foo(&mut candidates, 0, Mode::CO2, values);
    let co2_index = candidates.iter().enumerate()
        .find(|x| *x.1 == true)
        .map(|x| x.0)
        .unwrap();
    
    vec2num(&values[oxygen_index]) * vec2num(&values[co2_index])
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

    //println!("{}", part_1(&values));
    println!("{}", part_2(&values));

}
