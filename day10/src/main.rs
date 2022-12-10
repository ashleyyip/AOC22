
use std::env;
use std::fs;




fn day2(instructions:&Vec<Vec<String>> ) {
    let len = 240;
    let mut picture_vec: Vec<char> = vec!['.'; len];
    // println!("{:?}", picture_vec.into_iter().collect::<String>());

    let mut x: i32 = 1;
    let mut num_cycles = 1;

    let mut curPos: i32;

    for line in instructions {
        curPos = num_cycles-1;

        if curPos%40 == x-1 || curPos%40 == x || curPos%40 == x+1 {
            picture_vec[curPos as usize] = '#'; 
        }

        match line[0].as_str() {
            "noop" => {
                num_cycles += 1;
            }
            "addx" => {
                curPos += 1;
                if curPos%40 == x-1 || curPos%40 == x || curPos%40 == x+1 {
                    picture_vec[curPos as usize] = '#'; 
                }

                num_cycles += 2;
                x += line[1].parse::<i32>().unwrap();

            }
            _ => {}
        }        
    }

    println!("{:?}", picture_vec[0..40].into_iter().collect::<String>());
    println!("{:?}", picture_vec[40..80].into_iter().collect::<String>());
    println!("{:?}", picture_vec[80..120].into_iter().collect::<String>());
    println!("{:?}", picture_vec[120..160].into_iter().collect::<String>());
    println!("{:?}", picture_vec[160..200].into_iter().collect::<String>());
    println!("{:?}", picture_vec[200..].into_iter().collect::<String>());

}


fn day1(instructions:&Vec<Vec<String>> ) {
    let mut sum_strengths = 0;
    let mut x = 1;
    let mut num_cycles = 1;
    let mut checks: Vec<i32> = [220, 180, 140, 100, 60, 20].to_vec();
    let mut curVal = 0;
    
    for line in instructions {
        match line[0].as_str() {
            "noop" => {
                num_cycles += 1;
            }
            "addx" => {
                num_cycles += 2;
                curVal = line[1].parse::<i32>().unwrap();
                x += line[1].parse::<i32>().unwrap();
            }
            _ => {}
        }

        if !checks.is_empty() {
            if num_cycles == *checks.last().unwrap() {
                let cycle = checks.pop().unwrap();
                sum_strengths += cycle * x;
    
            }
            else if num_cycles -1 == *checks.last().unwrap() {
                let cycle = checks.pop().unwrap();
                sum_strengths += cycle * (x- curVal);
            }
        }
        
    }
    println!("{:?}", sum_strengths);

}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut instructions: Vec<Vec<String>> = contents.split("\n").map(|w| 
            w.split(' ').map(|x| x.parse().unwrap_or_default())
            .collect::<Vec<String>>())
        .collect();

    instructions.pop();
    // println!("{:?}", instructions);

    day1(&instructions);
    day2(&instructions);

}