use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let splitcontents = contents.split("\r\n\r\n");

    let mut sorted = splitcontents.map(sum_calories).collect::<Vec<i32>>();
    sorted.sort();
    sorted.reverse();
    let biggest = sorted.get(0).unwrap_or(&-1);
    println!("{biggest}");

    let biggest_three_sum: i32 = sorted.iter().take(3).sum();
    println!("{biggest_three_sum}");
}

fn sum_calories(calories: &str) -> i32 {
    calories
        .split("\r\n")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .sum()
    //.collect::<Vec<&str>>();
}
