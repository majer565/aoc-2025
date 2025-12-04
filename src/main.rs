use std::fs;

use crate::{day1::Dial, day2::IDReader, day3::BatteriesBank};

mod day1;
mod day2;
mod day3;

fn main() {
    // let mut dial: Dial = Dial::new();
    // let day1_pwd = dial.get_password();
    // let day2_pwd = IDReader::get_password();
    let day3_pwd = BatteriesBank::get_password();
    println!("PWD:: {}", day3_pwd);
}
