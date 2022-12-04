use std::fs;
use std::char;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\r\n").collect::<Vec<&str>>();

    // println!("{}", get_priority('a'));
    // println!("{}", get_priority('A'));
    // println!("{}", get_priority('Z'));

    let compartments = lines.iter().map(get_compartments);
    let shared_chars: Vec<char> = compartments.iter().map(find_shared_char).map(get_priority).collect();

    let priority_sum = priorities.iter().sum::<i32>();
    
    println!("{}", result)
}

fn get_compartments(bag: &str) -> (&str, &str) {
    return bag.split_at(bag.len()/2);
}

fn find_shared_char((a, b): (&str, &str)) -> char {
    let to_char_vec = |x: &str| -> Vec<char> {x.chars().collect::<Vec<char>>()};
    let (xs, ys) = (to_char_vec(a), to_char_vec(b));

    let mut shared_char = '.';

    for x in xs {
        if ys.contains(&x) {
            shared_char = x;
        }
    }

    if shared_char == '.' { panic!("Couldn't find common character!")}

    return shared_char;
}

fn get_priority(char: char) -> i32 {

    let correction = if char.is_ascii_uppercase() {-38} else {-96};

    let value = char as i8 + correction;

    value as i32
}