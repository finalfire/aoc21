use std::collections::HashMap;
use std::fs::*;

fn foo(lines: &Vec<Vec<u32>>, is_part_two: bool) -> usize {
    let mut h: HashMap<(u32, u32), u32> = HashMap::new();
    
    for v in lines {
        // no need for the three ifs
        // all three cases can be managed via x_off and y_off (e.g., x_off can be either 1, 0 or -1)
        if v[0] == v[2] {
            for j in v[1].min(v[3])..=v[1].max(v[3]) {
                let value = h.entry((v[0], j)).or_insert(0);
                *value += 1;
            }
        } else if v[1] == v[3] {
            for i in v[0].min(v[2])..=v[0].max(v[2]) {
                let value = h.entry((i, v[1])).or_insert(0);
                *value += 1;
            }
        } else if is_part_two {
            let x_off: i32 = if v[0] < v[2] { 1 } else { -1 };
            let y_off: i32 = if v[1] < v[3] { 1 } else { -1 };

            let (mut x, mut y) = (v[0] as i32, v[1] as i32);
            for _i in v[0].min(v[2])..=v[0].max(v[2]) {
                let value = h.entry((x as u32, y as u32)).or_insert(0);
                *value += 1;

                x += x_off;
                y += y_off;
            } 
        }
    }

    h.iter().filter(|(_k, v)| *v > &1).count()
}

fn main() {
    let input = read_to_string("inputs/day5/input")
        .expect("Cannot read the file");
    
    let lines: Vec<Vec<u32>> = input.lines().map(|line| {
        line.split(" -> ").map(|pair| {
            pair.split(',').map(|coord| {
                coord.parse::<u32>().unwrap()
            })
        }).flatten().collect::<Vec<u32>>()
    }).collect();

    println!("{}", foo(&lines, false));
    println!("{}", foo(&lines, true));
}
