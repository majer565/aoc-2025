use std::fs;

pub struct BatteriesBank;

impl BatteriesBank {
    pub fn get_password() -> u32 {
        // let input = BatteriesBank::read_input();
        let input = vec![String::from("546511191878915")];
        let mut sum: u32 = 0;

        for bank in input {
            sum += BatteriesBank::get_joltage_2(bank);
        }

        sum
    }

    fn get_joltage_2(bank: String) -> u32 {
        let radix = 10;
        let mut min_idx = 0;
        let mut chars_left = bank.len() as u32;
        let mut out_arr: Vec<u32> = Vec::new();

        for (idx, char_digit) in bank.char_indices() {
            let digit = match char_digit.to_digit(radix) {
                Some(d) => d,
                None => {
                    println!("Error cannot parse int");
                    break;
                }
            };

            if idx > 0 {
                if chars_left >= 12 {
                    min_idx = 0;
                } else {
                    min_idx = 12 - chars_left;
                }

                let mut insert_at = None;
                for (idx, &out_digit) in out_arr.iter().enumerate().skip(min_idx as usize) {
                    if digit > out_digit {
                        insert_at = Some(idx);
                        break;
                    }
                }

                match insert_at {
                    Some(idx) => {
                        // if idx == 0 {
                        //     for (out_id, out_dig) in out_arr.iter_mut().enumerate().skip(n)
                        // }
                        todo!("Insert or push");
                        out_arr.insert(idx + min_idx as usize, digit);
                        chars_left -= 1;
                    }
                    None => {
                        out_arr.push(digit);
                        chars_left -= 1;
                    }
                }
            } else {
                out_arr.insert(min_idx as usize, digit);
                println!("Inserted:: {}", &digit);
                println!("Out:: {:?}", &out_arr);
            }
        }

        0
    }

    fn get_joltage(bank: String) -> u32 {
        let radix = 10;
        let mut first: u32 = 0;
        let mut second: u32 = 0;
        let mut pik = bank.chars().peekable();

        while let Some(current_char) = pik.next() {
            let digit = match current_char.to_digit(radix) {
                Some(d) => d,
                None => {
                    println!("Error cannot parse int");
                    break;
                }
            };

            if let Some(&next) = pik.peek() {
                if digit > first {
                    first = digit;
                    second = 0;
                } else if digit > second {
                    second = digit;
                }
            } else {
                if digit > second {
                    second = digit;
                }
            }
        }

        first * 10 + second
    }

    fn read_input() -> Vec<String> {
        let file_path = "src/resources/day3.txt";
        let file_result = fs::read_to_string(file_path);
        let mut batteries_list = Vec::new();

        match file_result {
            Ok(file) => {
                for line in file.lines() {
                    batteries_list.push(line.trim().to_string());
                }
            }
            Err(err) => {
                println!("Error while reading file - {}", err);
            }
        }

        batteries_list
    }
}
