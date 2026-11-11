// mod backup; // Uncomment to pull backup working prototype code
use std::{io::stdin, num::ParseFloatError};

fn main() {
    let mut counter = 0;
    let num1 = loop {
        if counter == 0 {
            println!("Please input a number");
        } else {
            println!("Please input a valid number. ");
        }

        // init num1 as empty string for user input
        let mut num1 = String::new();

        // get user input.
        stdin().read_line(&mut num1).expect("Error reading line");

        let num1: f64 = match validate_num(num1) {
            Ok(num) => num,
            Err(_) => {
                println!("HELLO WORLD");
                counter += 1;
                continue;
            }
        };
        // println!("HELLO WORLD");
        break num1;
    };

    counter = 0; // reset counter to 0 for next loop
    let num2 = loop {
        if counter == 0 {
            println!("Please input next number");
        } else {
            println!("Please input a valid number");
        }

        // init num2 as empty string for user input
        let mut num2 = String::new();

        // get user input.
        stdin().read_line(&mut num2).expect("Error Reading line");

        let num2: f64 = match validate_num(num2) {
            Ok(num) => num,
            Err(_) => {
                counter += 1;
                continue;
            }
        };
        break num2;
    };

    println!("number 1 is {num1} and number 2 is {num2}.");
}

// fn pick_operator(num1: f64, num2: f64) {
//     todo!() // checkout inquire crate?? maybe be used.
// }

fn validate_num(num: String) -> Result<f64, ParseFloatError> {
    num.trim().parse()
}

fn validate_counter(counter: u32) {
    if counter == 0 {
        println!("Please input a number");
    } else {
        println!("Please input a valid number. ");
    }
}
