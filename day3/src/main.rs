use std::collections::HashMap;
use std::env;
use std::fs;

fn day1(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        let first: String = line[..line.len()/2].to_string();
        let second: String = line[line.len()/2..].to_string();
        
        let char_vec1: Vec<char> = first.chars().collect();          
        let char_vec2: Vec<char> = second.chars().collect();
        let mut common: u32 = 0;

        for ch in char_vec2 {
            if char_vec1.contains(&ch) {
                common = ch as u32;

            }
        }
        if common > 96 {
            common -= 96;
        }
        else {
            common -= 38;
        }
        sum += common;
    }

    return sum;
}

fn day2(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..lines.len()/3 {
        let first: &String = &lines[3*i];
        let second: &String = &lines[3*i+1];
        let third: &String = &lines[3*i+2];

        let char_vec1: Vec<char> = first.chars().collect();
        let char_vec2: Vec<char> = second.chars().collect();
        let char_vec3: Vec<char> = third.chars().collect();

        let mut common: Vec<char> = Vec::new();
        let mut commonChar: u32 = 0;

        for ch in char_vec2 {
            if char_vec1.contains(&ch) {
                common.push(ch)
            }
        }
        for ch in common {

            if char_vec3.contains(&ch) {
                commonChar = ch as u32;
            }
        }
        if commonChar > 96 {
            commonChar -= 96;
        }
        else {
            commonChar -= 38;
        }
        sum += commonChar;
    }

    return sum;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<String> =  contents.split("\n").map(|w| w.parse().unwrap_or_default()).collect();

    lines.pop();
    // println!("{:?}", lines);

    println!("{:?}", day1(&lines));
    println!("{:?}", day2(&lines));

}
