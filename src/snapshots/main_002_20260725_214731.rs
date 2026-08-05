use crate::guess_game::get_user_number;

fn main() {
    println!("\n");

    let user_number_result = get_user_number();
    let user_number = match user_number_result {
        Ok(num) => num,
        Err(err) => panic!("Error: {}", err),
    };

    println!(" -> user number is: {}", user_number);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!("\n -> The End ...\n");
}

mod guess_game {
    use std::{
        error::Error,
        io::{Read, Write},
    };

    pub fn get_user_number() -> Result<u32, Box<dyn Error>> {
        print!(" -> enter a number: ");
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err)),
        }

        println!("\n -> step 1\n");

        let mut input_number = String::new();
        match std::io::stdin().read_to_string(&mut input_number) {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err)),
        }

        println!("\n -> step 2\n");

        match input_number.trim().parse::<u32>() {
            Ok(num) => Ok(num),
            Err(err) => Err(Box::new(err)),
        }
    }
}
