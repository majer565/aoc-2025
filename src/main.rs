use std::fs;

use crate::utils::Dial;

mod utils;

fn read_code() -> Vec<String> {
    let file_path = "src/code.txt";
    let file_result = fs::read_to_string(file_path);
    let mut code_list = Vec::new();

    match file_result {
        Ok(file) => {
            for line in file.lines() {
                code_list.push(line.to_string());
            }
        }
        Err(err) => {
            println!("Error while reading file - {}", err);
        }
    }

    code_list
}

fn main() {
    // let input: Vec<&str> = vec![
    //     "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    // ];
    let input: Vec<String> = read_code();
    let mut dial: Dial = Dial::new();

    for step in input {
        dial.rotate(step.as_str());
    }

    println!("The password is - {}", &dial.get_over_zero());
}
