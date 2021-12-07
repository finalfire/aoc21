use std::fs::*;

fn part_1(values: &Vec<i32>) -> i32 {
    let mut my_values = values.clone();
    my_values.sort();
    
    let median = my_values[my_values.len() / 2];
    my_values.iter()
        .map(|value| (value - median).abs())
        .sum()
}

fn part_2(values: &Vec<i64>) -> i64 {
    let mut my_values = values.clone();
    my_values.sort();
    
    let avg: i64 = my_values.iter().sum::<i64>() / my_values.len() as i64;
    my_values.iter()
        .map(|value| {
            let n = (value - avg).abs();
            (n * (n + 1)) / 2
        })
        .sum()


}

fn main() {
    let input = read_to_string("inputs/day7/input")
        .expect("Cannot read the file");

    let values: Vec<i64> = input.split(',')
        .map(|value| value.parse::<i64>().unwrap())
        .collect();
    
    println!("{}", part_1(&values));
    println!("{}", part_2(&values));
}
