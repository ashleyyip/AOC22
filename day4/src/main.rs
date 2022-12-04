use std::env;
use std::fs;

fn day2(lines: &Vec<Vec<i32>>) -> i32 {
    let mut numOverlaps: i32 = 0;

    for pair in lines {
        if pair[0] >= pair[2] && pair[0] <= pair[3] {
            numOverlaps += 1;
        }
        else if pair[1] >= pair[2] && pair[1] <= pair[3] {
            numOverlaps += 1;
        }
        else if pair[2] >= pair[0] && pair[2] <= pair[1] {
            numOverlaps += 1;
        }
        else if pair[3] >= pair[0] && pair[3] <= pair[1] {
            numOverlaps += 1;
        }
    }
    return numOverlaps;
}


fn day1(lines: &Vec<Vec<i32>>) -> i32 {
    let mut numPairs: i32 = 0;

    for pair in lines {
        // first pair fits into second
        if pair[0] >= pair[2] && pair[1] <= pair[3] {
            numPairs += 1;
        }
        else if pair[2] >= pair[0] && pair[3] <= pair[1] { // second fits into first
            numPairs += 1;
        }
    }
    return numPairs;
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<Vec<i32>> =  contents.split("\n").map(|w| 
        w.split(&['-', ','][..])
            .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<i32>>())
    .collect();

    lines.pop();
    // println!("{:?}",lines);

    println!("{:?}", day1(&lines));
    println!("{:?}", day2(&lines));
}