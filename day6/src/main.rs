
use std::collections::HashSet;
use std::env;
use std::fs;

fn day12(chars: &Vec<char>, num_distinct: usize) {
    for i in 0..chars.len()-(num_distinct-1) {
        let char_sub = &chars[i..i+num_distinct];
        let char_set: HashSet<char> = char_sub.iter().cloned().collect();

        if char_set.len() == num_distinct {
            println!("{:?}", i+num_distinct);
            return;
        }
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let chars: Vec<char> =  contents.chars().collect();
    day12(&chars, 4);
    day12(&chars, 14);

}