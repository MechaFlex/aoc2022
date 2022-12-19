use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\r\n").collect::<Vec<&str>>();

    let tuples: Vec<(&str, &str)> = lines.iter().cloned().map(get_tuples).collect();
    let surround_count = tuples.iter().cloned().map(is_surrounded).sum();
}

fn get_tuples(line: &str) -> (&str, &str) {
    line.split_once(',').unwrap()
}

fn is_surrounded((a,b): (&str,&str)) -> bool {
    
}