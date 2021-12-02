use std::fs::*;

enum Part { One, Two, }

fn solve(input: String, part: Part) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut hpos = 0;

    input.lines().for_each(|line| {
        let (mode, value_str) = line.split_once(' ').expect("Cannot parse row");
        let value: i32 = value_str.parse().expect("Cannot parse value");

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

    // a single function does the trick (in part one, aim is our depth)
    // the drawback in part 1 is that we calculate depth anyway
    match part {
        Part::One => aim * hpos,
        Part::Two => depth * hpos
    }
}

fn main() {
    let input = read_to_string("inputs/day2/input")
        .expect("Cannot read the file");
    
    println!("{}", solve(input.clone(), Part::One));
    println!("{}", solve(input.clone(), Part::Two));
}
