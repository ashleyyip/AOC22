use std::env;
use std::fs;
// use std::cmp;

fn calc_score(opponent: u32, you: u32) -> u32 {
    let mut score: u32 = 0;

    if opponent == 1 && you == 3 {
        score = you;
    }
    else if opponent == 3 && you == 1 {
        score = 7;
    }
    else if opponent < you {
        score = 6 + you;
    }
    else if opponent == you {
        score = 3 + you;
    }
    else if opponent > you {
        score = you
    }
    else {
        println!("unhandled");

    }
    return score
}

fn day1(lines: &Vec<Vec<char>>) -> u32 {
    let mut score: u32 = 0;
    for game in lines {
        let opponent = game[0] as u32 - 64;
        let you = game[1] as u32 - (23+64);

        score += calc_score(opponent, you);


        // println!("{:?} {:?} {:?}", opponent, you, score);


    }
    return score;
}

fn day2(lines: &Vec<Vec<char>>) -> u32 {
    let mut score: u32 = 0;
    for game in lines {
        let opponent = game[0] as u32 - 64;
        let required = game[1] as u32 - (23+64);
        // let mut you: u32 = 0;

        if required == 1 {
            if opponent == 1 {
                score += calc_score(opponent, 3)
            }
            else {
                score += calc_score(opponent, opponent-1)

            }
        }
        else if required == 2 {
            score += calc_score(opponent, opponent) // have to play the same thing as your opponent
        }
        else if required == 3 {
            if opponent == 3 {
                score += calc_score(opponent, 1)
            }
            else {
                score += calc_score(opponent, opponent+1) // 1 needs 2, 2 needs 3, 3 needs 1
            }
        }
    }
    return score;
}

fn main() {

    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<Vec<char>> =  contents.split("\n").map(|w| 
        w.split(" ")
            .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<char>>())
    .collect();

    lines.pop();

    println!("{:?}", day1(&lines));
    println!("{:?}", day2(&lines));

}
