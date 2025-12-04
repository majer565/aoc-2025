use std::fs;

pub struct BatteriesBank;

impl BatteriesBank {
    pub fn get_password() -> usize {
        let input = BatteriesBank::read_input();
        let mut sum: usize = 0;

        for bank in input {
            sum += BatteriesBank::get_joltage_3(bank);
        }

        sum
    }

    fn get_joltage_3(bank: String) -> usize {
        let radix = 10;
        let mut result: usize = 0;
        let char_arr: Vec<char> = bank.chars().collect::<Vec<_>>();

        let mut highest_digit_index: i64 = -1;

        for i in 0..12 {
            let end: i64 = char_arr.len() as i64 - 11 + i as i64;

            let mut highest_digit = 0;
            for idx in (highest_digit_index + 1)..end {
                if let Some(char_digit) = char_arr.get(idx as usize) {
                    if let Some(digit) = char_digit.to_digit(radix) {
                        if digit > highest_digit {
                            highest_digit = digit;
                            highest_digit_index = idx;
                        }
                    }
                }
            }
            result *= 10;
            result += highest_digit as usize;
        }

        result
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
