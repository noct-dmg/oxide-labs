// mod backup; // uncomment to pull in backup code.

fn main() {
    //-------------STATEMENTS AND EXPRESSIONS-------------
    // let y = 6; // this is a statement

    // let x = (let 6 = 6); // this wont compile, only assign expressions to variables

    let y = {
        let _x = 3;
    };
    println!("{:?}", std::any::type_name_of_val(&y));
}
