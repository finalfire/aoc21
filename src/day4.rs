use std::fs::*;

const N: usize = 5;
const EMPTY: (i8,i8) = (-1,-1);

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<u32>>,
    rows: Vec<u32>,
    cols: Vec<u32>,
    hm: Vec<(i8, i8)>    
}

impl Board {
    fn new(input: &Vec<&str>, max_draw: &u8) -> Board {
        let mut hm = vec![(-1,-1); *max_draw as usize + 1];

        let grid: Vec<Vec<u32>> = input.iter().enumerate().map(|(i, row)| {
            row.split_whitespace().enumerate()
                .map(|(j, value)| {
                    let v = value.parse::<u32>().unwrap();
                    hm[v as usize] = (i as i8, j as i8);
                    v
                })
                .collect()
        }).collect();

        let rows: Vec<u32> = grid.iter().map(|row| row.iter().sum()).collect();
        let cols: Vec<u32> = (0..N).map(|j| (0..N).map(|i| grid[i][j]).sum()).collect();

        Board { grid, rows, cols, hm }
    }
}

fn foo(to_draw: &Vec<u8>, boards: &mut Vec<Board>, max_draw: u32) -> (u32, u32) {
    let (mut final_score_1, mut final_score_2) = (0, 0);
    let mut winners = vec![false; boards.len()];
    let mut first = true;

    for d in to_draw {
        for b_i in 0..boards.len() {
            if winners[b_i] {
                continue;
            }

            if boards[b_i].hm[*d as usize] != EMPTY {
                let i = boards[b_i].hm[*d as usize].0 as usize;
                let j = boards[b_i].hm[*d as usize].1 as usize;

                boards[b_i].grid[i][j] = max_draw as u32 + 1;
                boards[b_i].rows[i] -= *d as u32;
                boards[b_i].cols[j] -= *d as u32;

                if boards[b_i].rows[i] == 0 || boards[b_i].cols[j] == 0 {
                    let score: u32 = boards[b_i].rows.iter().sum();
                    final_score_2 = *d as u32 * score;
                    winners[b_i] = true;

                    if first {
                        final_score_1 = final_score_2;
                        first = false;
                    }
                }
            }
        }
    }

    (final_score_1, final_score_2)
}

fn main() {
    let input = read_to_string("inputs/day4/input")
        .expect("Cannot read the file");

    let mut lines_it = input.lines();
    let to_draw: Vec<u8> = lines_it.next().map(|line|
        line.split(',').map(|value| value.parse::<u8>().unwrap()).collect()
    ).expect("Cannot parse numbers to draw");
    let max_draw = to_draw.iter().max().unwrap();

    let mut boards: Vec<Board> = Vec::new();

    let mut board_it = lines_it.peekable();
    while board_it.peek().is_some() {
        let board_input: Vec<&str> = board_it.by_ref().skip(1).take(N).collect();
        boards.push(Board::new(&board_input, &max_draw));
    }

    let (first_winner, last_winner) = foo(&to_draw, &mut boards, *max_draw as u32);
    println!("{} {}", first_winner, last_winner);
}