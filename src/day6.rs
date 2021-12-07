use std::fs::*;

fn foo(fishes: &mut Vec<i32>, n: &usize) -> usize {
    // counts[i] = n. of fishes at the state i
    let mut counts = vec![0usize; 9];
    fishes.iter().for_each(|fish|
        counts[*fish as usize] += 1);
    
    // rotate 1 step left and update counts[6] according to count[0]
    for _d in 0..*n {
        counts = counts.iter().enumerate()
            .map(|(i, _v)| {
                counts[(i + 1 as usize).rem_euclid(9)]
                    + if i == 6 { counts[0] } else { 0 }
            })
            .collect();
    }
    
    counts.iter().sum()
}

fn main() {
    let input = read_to_string("inputs/day6/input")
        .expect("Cannot read the file");

    let mut fishes: Vec<i32> = input.split(',')
        .map(|fish| fish.parse::<i32>().unwrap())
        .collect();
    
    println!("{}", foo(&mut fishes, &80));
    println!("{}", foo(&mut fishes, &256));
}
