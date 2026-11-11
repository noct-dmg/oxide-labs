use std::{io::stdin, num::ParseFloatError};

fn main() {
    let num1 = loop {
        println!("Please input a number");
        let mut num1 = String::new();

        stdin().read_line(&mut num1).expect("Error reading line");

        let num1: f64 = match validate_num(num1) {
            Ok(num) => num,
            Err(_) => continue,
        };
        break num1;
    };

    let num2 = loop {
        println!("Please input next number");
        let mut num2 = String::new();

        stdin().read_line(&mut num2).expect("Error Reading line");

        let num2: f64 = match validate_num(num2) {
            Ok(num) => num,
            Err(_) => continue,
        };
        break num2;
    };
}

fn pick_operator(num1: f64, num2: f64) {
    todo!() // checkout inquire crate?? maybe be used.
}

fn validate_num(num: String) -> Result<f64, ParseFloatError> {
    num.parse()
}
