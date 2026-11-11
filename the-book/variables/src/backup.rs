use std::{any::type_name_of_val, io::stdin};

pub fn _backup_code() {
    // testing shadowing of variabls
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of X is {x}\n");

    // constant variable
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // testing printing just variable type
    let y: f32 = 3.2;
    let z = 2.0;
    println!(
        "y is data type {}, and z is data type {}.\n",
        std::any::type_name_of_val(&y),
        std::any::type_name_of_val(&z)
    );

    //  boolean types
    let t = true;

    let f: bool = false;

    println!(
        "The value of T is {t} and its data type is {}",
        type_name_of_val(&t)
    );
    println!(
        "The value of F is {f} and its data type is {}\n",
        type_name_of_val(&f)
    );

    // The character type
    let c = 'z';
    let z: char = 'Z'; // explicit annotation
    let heart_eyed_cat = '😻';

    println!(
        "The values & data types of C, Z, and heart_eyed_cat are as follows:\n{c} -> {}, {z} -> {}, and {heart_eyed_cat} -> {}\n",
        type_name_of_val(&c),
        type_name_of_val(&z),
        type_name_of_val(&heart_eyed_cat)
    );
    // messing with compound types
    //-------------TUPLES-------------
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let tup2: (&str, i32, f64) = ("testing123", 24, 2.1);

    let a = tup2.0;
    let b = tup2.1;
    let c = tup2.2;

    println!(
        "The tuple tup2: {:?} was broken up into multiple values.\n
        A -> {}\n
        B -> {}\n
        C -> {}",
        tup2, a, b, c
    );
}

pub fn _pick_a_month() {
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut counter = 0; // count for loops to make sure prints properly

    // get a month in int from user.
    let picked_month = loop {
        if counter == 0 {
            println!("Please pick a month of the year using numbers.");
        } else {
            println!("Please pick a valid month");
        }

        let mut picked_month = String::new(); // new empty string

        stdin() // get string value from user
            .read_line(&mut picked_month)
            .expect("ERROR READING LINE");

        // Convert user string to u32 value
        let picked_month: u32 = match picked_month.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                counter += 1;
                continue;
            }
        };

        // make sure its same length as months array.
        if picked_month > months.len() as u32 {
            counter += 1;
            continue;
        }
        break picked_month;
    };

    // using shadow'd picked_month variable to pull month from array
    let picked_month = months[picked_month as usize - 1];

    println!("You picked {picked_month}");
}
