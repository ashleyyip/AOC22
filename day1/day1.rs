use std::env;
use std::fs;
use std::cmp;


fn main() {
    let mut sums: Vec<i32> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");


    let v: Vec<String> = contents.split("\n\n").filter_map(|w| w.parse().ok()).collect();
    // println!("{:?}", v);
    for item in v {
        // println!("{:?}", item);

        let v2: Vec<i32> = item.split("\n").filter_map(|w| w.parse().ok()).collect();
        let cur_sum: i32 = v2.iter().sum();
        sums.push(cur_sum);



    }
    sums.sort_by(|a, b| b.cmp(a));
    
    println!("{:?}", sums);
    
    let top_three = sums[0] + sums[1] + sums[2];
    println!("{:?}", top_three);

    

        // ;
    // let v: Vec<i32> = contents.split("\n\n").filter_map(|w| w.parse().ok()).collect();
    // let v: Vec<i32> = contents.split("\n\n").filter_map(|w| w.parse().ok());

    //  let v: Vec<i32> = contents.split("\n").filter_map(|w| w.parse().ok()).map(|x: i32| x/3 - 2).collect();
    //  let tot: i32 = v.iter().sum();
    //  println!("{:?}", tot);
}
