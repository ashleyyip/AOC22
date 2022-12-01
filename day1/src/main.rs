use std::env;
use std::fs;
// use std::cmp;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut sums: Vec<i32> = contents.split("\n\n").map(|w| 
        w.split("\n")
            .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<i32>>().iter().sum())
    .collect();

    sums.sort_by(|a, b| b.cmp(a));
    
    println!("{:?}", sums);
    let top_three = sums[0] + sums[1] + sums[2];
    println!("{:?}", top_three);

}
