use crate::utils::Dial;

mod utils;

fn main() {
    let input: Vec<&str> = vec![
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];
    let mut dial: Dial = Dial::new();
    let mut password: u16 = 0;

    for step in input {
        if dial.rotate(step) == 0 {
            password += 1;
        }
    }

    println!("The password is - {}", &password);
}
