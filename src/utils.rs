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

    pub fn get_over_zero(&self) -> i32 {
        self.over_zero
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

    fn rotate_dial(&mut self, steps: i32, dir: Dir) {
        let mut result = 0;

        match dir {
            Dir::RIGHT => {
                result = &self.point + steps;
                if result > 100 {
                    let over_zero_rotations = result / 100;
                    self.over_zero += over_zero_rotations;
                }

                self.point = (&self.point + steps).rem_euclid(100);
            }
            Dir::LEFT => {
                result = &self.point - steps;
                if result < 0 && self.point != 0 {
                    let over_zero_rotations = (result - 100).abs() / 100;
                    todo!();

                    self.over_zero += over_zero_rotations;
                }

                self.point = (&self.point - steps).rem_euclid(100);
            }
        }

        // println!("New point is: {}", &self.point);

        if self.point == 0 {
            self.over_zero += 1;
        }
    }
}
