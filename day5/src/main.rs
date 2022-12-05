
use std::env;
use std::fs;
use std::num;
use regex::Regex;


fn day2(stacks: &Vec<Vec<char>>, instructions:&Vec<Vec<i32>> ) {
    
    for line in instructions {
        let move_num: i32 = line[0];
        let from: usize = (line[1]-1).try_into().unwrap();
        let to: usize = (line[2]-1).try_into().unwrap();

        let mut queue: Vec<char> = Vec::new();
        for i in 0..move_num {
            queue.push(stacks[from].pop().unwrap());
        }
        queue.reverse();
        stacks[to].append(&mut queue);
    }

    for stack in stacks {
        println!("{:?}", stack.last().unwrap());
    }
}


fn day1(stacks: &Vec<Vec<char>>, instructions:&Vec<Vec<i32>> ) {
    
    for line in instructions {
        let move_num: i32 = line[0];
        let from: usize = (line[1]-1).try_into().unwrap();
        let to: usize = (line[2]-1).try_into().unwrap();

        for i in 0..move_num {
            let moving = stacks[from].pop().unwrap();
            stacks[to].push(moving);
        }
    }

    for stack in stacks {
        println!("{:?}", stack.last().unwrap());
    }

}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");


    let input: Vec<String> = contents.split("\n\n").map(|w| w.parse().unwrap_or_default()).collect();
    // println!("{:?}", input[0]);

    let re = regex::Regex::new(r"[A-Z]|    ").unwrap();
    

    let mut starting: Vec<Vec<String>> = input[0].split("\n").map(|w| 
       re.find_iter(w)
       .filter_map(|digits| digits.as_str().parse().ok())
       .collect())
    .collect();

    starting.pop();
    
    println!("{:?}", starting);
    // println!("{:?}", numStacksLine);

    let numStacks: usize = starting[0].len();

    println!("{:?}", numStacks);

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(numStacks);

    for i in 0..numStacks {
        stacks.push(Vec::new());
    }
    println!("{:?}", stacks);

    for line in starting {
        for i in 0..numStacks {
            if line[i] != "    " {
                stacks[i].push(line[i].chars().nth(0).unwrap());
            }
        }
    }
    // println!("{:?}", stacks);

    for i in 0..numStacks {
        stacks[i].reverse();
    }
    println!("{:?}", stacks);

    // println!("{:?}", input[1]);

    let mut instructions: Vec<Vec<i32>> = input[1].split("\n").map(|w| 
            w.split(' ')
                .map(|x| x.parse().unwrap_or_default())
                .filter(|y| *y != 0)
            .collect::<Vec<i32>>())
        .collect();

    instructions.pop();
    println!("{:?}", instructions);


    day1(&stacks, &instructions);
    day2(&stacks, &instructions);

}