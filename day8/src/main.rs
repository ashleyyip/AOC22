
use std::env;
use std::fs;
use std::collections::HashMap;


fn star1(instructions:&Vec<Vec<i32>> ) {

    let mut visible_map: HashMap<(usize, usize), bool> = HashMap::new();
    let mut num_visible = 0;
    let mut cur_max: i32 = -1;

    for i in 0..instructions.len() {
        cur_max = -1;
        for j in 0..instructions[0].len() {
            if instructions[i][j] > cur_max {
                visible_map.insert((i, j), true);
                cur_max = instructions[i][j];
                num_visible += 1;
            }
        }
        cur_max = -1;
        for j in (0..instructions[0].len()).rev() {
            if instructions[i][j] > cur_max {
                cur_max = instructions[i][j];
                if !visible_map.contains_key(&(i, j)) {
                    visible_map.insert((i, j), true);
                    num_visible += 1;
                }
                
            }
        }
    }

    for j in 0..instructions[0].len() {
        cur_max = -1;
        for i in 0..instructions[0].len() {
            if instructions[i][j] > cur_max {
                cur_max = instructions[i][j];
                if !visible_map.contains_key(&(i, j)) {
                    visible_map.insert((i, j), true);
                    num_visible += 1;
                }
                
            }
        }
        cur_max = -1;
        for i in (0..instructions[0].len()).rev() {
            if instructions[i][j] > cur_max {
                cur_max = instructions[i][j];
                if !visible_map.contains_key(&(i, j)) {
                    visible_map.insert((i, j), true);
                    num_visible += 1;
                } 
            }
        }
    }
    println!("{:?}", num_visible);

}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut instructions: Vec<Vec<i32>> = contents.split("\n")
            .map(|w| w.chars().map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>())
        .collect();

    instructions.pop();
    println!("{:?}", instructions);

    star1(&instructions);

}