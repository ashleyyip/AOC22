
use std::env;
use std::fs;
use std::collections::HashSet;


fn check_tail(head_x: i32, head_y: i32, mut tail_x: i32, mut tail_y: i32) -> (i32, i32) {
    if (head_x - tail_x).abs() == 2 {
        tail_y = head_y;
        if head_x > tail_x {
            tail_x += 1;
        }
        else if head_x < tail_x {
            tail_x -= 1;
        }
    }
    else if (head_y - tail_y).abs() == 2 {
        tail_x = head_x;
        if head_y > tail_y {
            tail_y += 1;
        }
        else if head_y < tail_y {
            tail_y -= 1;
        }
        
    }
    return (tail_x, tail_y);
}

fn star1(instructions:&Vec<Vec<String>> ) {
    
    let mut num_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;

    for line in instructions {
        let num_steps = line[1].parse::<i32>().unwrap();
        println!("{:?}", line);

        match line[0].chars().next().unwrap() {
            'R' => {
                for i in 1..num_steps+1 {
                    (tail_x, tail_y) = check_tail(head_x+i, head_y, tail_x, tail_y);
                    num_visited.insert((tail_x, tail_y));
                }
                head_x += num_steps;
            }
            'L' => {
                for i in 1..num_steps+1 {
                    (tail_x, tail_y) = check_tail(head_x-i, head_y, tail_x, tail_y);
                    num_visited.insert((tail_x, tail_y));
                }
                head_x -= num_steps;
                
            }
            'U' => {
                for i in 1..num_steps+1 {
                    (tail_x, tail_y) = check_tail(head_x, head_y+i, tail_x, tail_y);
                    num_visited.insert((tail_x, tail_y));
                }
                head_y += num_steps;
            }
            'D' => {
                for i in 1..num_steps+1 {
                    (tail_x, tail_y) = check_tail(head_x, head_y-i, tail_x, tail_y);
                    num_visited.insert((tail_x, tail_y));
                }
                head_y -= num_steps;
            }
            _ => {

            }
        }
        // println!("{:?}, {:?}", head_x, head_y);
        // println!("{:?}, {:?}", tail_x, tail_y);
        // println!("{:?}", num_visited);
    }

    // println!("{:?}", num_visited);
    println!("{:?}", num_visited.len());
    

}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut instructions: Vec<Vec<String>> = contents.split("\n").map(|w| 
        w.split_whitespace()
            .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<String>>())
    .collect();

    instructions.pop();
    println!("{:?}", instructions);

    star1(&instructions);
    // day2(&stacks, &instructions);

}