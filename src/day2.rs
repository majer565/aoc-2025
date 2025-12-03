use std::fs;

pub struct IDReader;

impl IDReader {
    pub fn get_password() -> i64 {
        let input = IDReader::read_intervals();
        let mut sum: i64 = 0;

        for interval in input {
            let invalid_list = IDReader::get_invalid_ids(interval);
            sum += invalid_list.iter().sum::<i64>();
        }

        sum
    }

    fn read_intervals() -> Vec<String> {
        let file_path = "src/resources/day2.txt";
        let file_result = fs::read_to_string(file_path);
        let mut interval_list = Vec::new();

        match file_result {
            Ok(file) => {
                interval_list = file.trim().split(",").map(|int| int.to_string()).collect();
            }
            Err(err) => {
                println!("Error while reading file - {}", err);
            }
        }

        interval_list
    }

    fn get_invalid_ids(interval: String) -> Vec<i64> {
        let id_list = interval
            .split("-")
            .map(|int| int.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let start_id = id_list[0];
        let end_id = id_list[1];
        let mut invalid_id_list: Vec<i64> = Vec::new();

        for id in start_id..=end_id {
            let id_str = id.to_string();
            let mut temp_str = String::new();

            for c in id_str.chars() {
                temp_str.push(c);
                let matches: Vec<&str> = id_str.matches(&temp_str).collect();

                if matches.len() > 1
                    && matches.len()
                        == (id_str.len() / temp_str.len() + (id_str.len() % temp_str.len()))
                {
                    invalid_id_list.push(id);
                    break;
                }
            }
        }

        invalid_id_list
    }
}
