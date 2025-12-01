pub struct Dial {
    point: i16,
}

enum Dir {
    LEFT,
    RIGHT,
}

impl Dial {
    pub fn new() -> Self {
        Dial { point: 50 }
    }

    pub fn rotate(&mut self, code: &str) -> i16 {
        match code.chars().next().unwrap() {
            'R' => {
                let steps_as_str = &code[1..];
                match steps_as_str.parse::<i16>() {
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
                match steps_as_str.parse::<i16>() {
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

        self.point
    }

    fn rotate_dial(&mut self, steps: i16, dir: Dir) {
        match dir {
            Dir::RIGHT => {
                self.point = (&self.point + steps) % 99;
            }
            Dir::LEFT => {
                self.point = (&self.point - steps) % 99;
            }
        }

        println!("New point is: {}", &self.point);
    }
}
