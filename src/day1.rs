use std::fs;

pub struct Dial {
    point: i32,
    over_zero: i32,
}

enum Dir {
    LEFT,
    RIGHT,
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            point: 50,
            over_zero: 0,
        }
    }

    pub fn get_password(&mut self) -> i32 {
        let input: Vec<String> = self.read_code();

        for step in input {
            self.rotate(step.as_str());
        }

        self.get_over_zero()
    }

    pub fn rotate(&mut self, code: &str) -> i32 {
        match code.chars().next().unwrap() {
            'R' => {
                let steps_as_str = &code[1..];
                match steps_as_str.parse::<i32>() {
                    Ok(steps) => {
                        self.rotate_dial(steps, Dir::RIGHT);
                    }
                    Err(err) => {
                        println!("Err {}", err);
                    }
                }
            }
            'L' => {
                let steps_as_str = &code[1..];
                match steps_as_str.parse::<i32>() {
                    Ok(steps) => {
                        self.rotate_dial(steps, Dir::LEFT);
                    }
                    Err(err) => {
                        println!("Err {}", err);
                    }
                }
            }
            _ => println!("Error!"),
        }

        self.over_zero
    }

    fn get_over_zero(&self) -> i32 {
        self.over_zero
    }

    fn read_code(&self) -> Vec<String> {
        let file_path = "src/resources/code.txt";
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

    fn rotate_dial(&mut self, steps: i32, dir: Dir) {
        let mut result = 0;

        match dir {
            Dir::RIGHT => {
                result = &self.point + steps;
                if result > 99 {
                    let over_zero_rotations = result / 100;

                    self.over_zero += over_zero_rotations;
                }

                self.point = (&self.point + steps).rem_euclid(100);
            }
            Dir::LEFT => {
                result = &self.point - steps;
                if result < 0 {
                    let over_zero_rotations = (result.abs()) / 100;

                    self.over_zero += over_zero_rotations;
                }
                if result <= 0 && self.point != 0 {
                    self.over_zero += 1;
                }

                self.point = (&self.point - steps).rem_euclid(100);
            }
        }
    }
}
