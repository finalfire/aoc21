use std::fs::*;

fn part_1(values: &Vec<u32>) -> usize {
    values.windows(2).filter(|s| s[1] > s[0]).count()
}

fn part_2(values: &Vec<u32>) -> usize {
    values.windows(4).filter(|s| s[3] > s[0]).count()
}

fn main() {
    let input_1 = read_to_string("inputs/day1/input")
        .expect("Cannot read the file");
    let values: Vec<u32> = input_1.lines()
        .map(|line| line.parse().expect("Cannot parse"))
        .collect();
    
    println!("Part 1: {}", part_1(&values));
    println!("Part 2: {}", part_2(&values));
}
