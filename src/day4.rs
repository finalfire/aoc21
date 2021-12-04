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

fn part_1(to_draw: &Vec<u8>, boards: &mut Vec<Board>, max_draw: u32) -> u32 {
    let mut winner = false;
    let mut final_score = 0;

    for d in to_draw {
        for b in &mut *boards {
            if b.hm[*d as usize] != EMPTY {
                let i = b.hm[*d as usize].0 as usize;
                let j = b.hm[*d as usize].1 as usize;

                b.grid[i][j] = max_draw as u32 + 1;
                b.rows[i] -= *d as u32;
                b.cols[j] -= *d as u32;

                if b.rows[i] == 0 || b.cols[j] == 0 {
                    winner = true;
                    let score: u32 = b.rows.iter().sum();
                    final_score = *d as u32 * score;
                    break;
                }


            }
        }

        if winner {
            break;
        }
    }

    final_score
}

fn part_2(to_draw: &Vec<u8>, boards: &mut Vec<Board>, max_draw: u32) -> u32 {
    let mut final_score = 0;
    let mut winners = vec![false; boards.len()];

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
                    final_score = *d as u32 * score;
                    winners[b_i] = true;
                }
            }
        }
    }

    final_score
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

    //println!("{}", part_1(&to_draw, &mut boards, *max_draw as u32));
    //println!("{}", part_2(&to_draw, &mut boards, *max_draw as u32));
}