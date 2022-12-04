use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\r\n").collect::<Vec<&str>>();

    let sum_of_games = lines.iter().map(|line| -> (&str, &str) {
        let characters = line.split(' ').take(2).collect::<Vec<&str>>();

        if let [first, second, ..] = &characters[..] {
            return (first, second);
        } else {
            panic!("This shouldn't happen!")
        }
    }).map(|(a,b )| (match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Couldn't match A, B or C")
    }, match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Couldn't match X, Y or Z")
    })).map(evaluate_score).sum::<i32>();

    println!("{}", sum_of_games)

}

fn evaluate_score((a, b): (i32, i32)) -> i32 {

    // PART 1
    // let mut score = b;

    // match a-b {
    //     0 => score += 3, //draw
    //     1 | -2 => score += 0, //loss
    //     2 | -1 => score += 6, //win
    //     b => panic!("non-valid pattern {}", b),
    // }
    
    // return score;

    // PART 2
    let mut score = (b-1)*3;

    if b == 2 { score += a}
    else if b == 1 { score += ((a+1) % 3) + 1}
    else if b == 3 { score += (a % 3) + 1}
    else { panic!("Couldn't find a match") }

    return score;
}