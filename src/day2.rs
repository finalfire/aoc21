use std::fs::*;

fn part_1(input: String) -> i32 {
    let mut depth = 0;
    let mut hpos = 0;

    input.lines().for_each(|line| {
        let mut it = line.split_whitespace();
        let mode = it.next().expect("Cannot parse mode");
        let value: i32 = it.next().unwrap().parse().expect("Cannot parse value");

        match mode {
            "forward" => hpos += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => {}
        };
    });

    depth * hpos
}

fn part_2(input: String) -> i32 {
    let mut depth = 0;
    let mut hpos = 0;
    let mut aim = 0;
    
    input.lines().for_each(|line| {
        let mut it = line.split_whitespace();
        let mode = it.next().expect("Cannot parse mode");
        let value: i32 = it.next().unwrap().parse().expect("Cannot parse value");

        match mode {
            "down" => aim += value,
            "up" => aim -= value,
            "forward" => {
                hpos += value;
                depth += aim * value;
            },
            _ => {}
        };
    });

    depth * hpos
}

fn main() {
    let input = read_to_string("inputs/day2/input")
        .expect("Cannot read the file");
    
    println!("{}", part_1(input.clone()));
    println!("{}", part_2(input.clone()));
}
