fn main() {
    println!("\n");

    let mut gm = guess_game::Database::new(0..=100);

    'game: loop {
        let user_number_result = guess_game::get_user_number();
        let user_number = match user_number_result {
            Ok(user_num) => user_num,
            Err(err) => panic!("Error: {}", err),
        };

        gm.set_user_number(user_number);

        let compare_result = gm.compare(user_number);
        match compare_result {
            Ok(msg) => {
                println!("{}", msg);
                break 'game;
            }
            Err(msg) => println!("{}", msg),
        }
    }

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    for item in gm.get_user_numbers().iter() {
        print!(" {}", item);
    }
    println!();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!("\n -> The End ...\n");
}

mod guess_game {

    use rand::{RngExt, rng};
    use std::{error::Error, io::Write, ops::RangeInclusive};

    pub struct Database {
        random_number: u32,
        user_numbers: Vec<u32>,
    }

    impl Database {
        pub fn new(range: RangeInclusive<u32>) -> Database {
            Database {
                random_number: rng().random_range(range),
                user_numbers: Vec::new(),
            }
        }

        pub fn set_user_number(&mut self, num: u32) {
            self.user_numbers.push(num);
        }

        pub fn get_user_numbers(&self) -> &Vec<u32> {
            &self.user_numbers
        }

        pub fn compare(&self, num: u32) -> Result<&str, &str> {
            if self.random_number > num {
                Err(" -> your number is less than chosen number ...")
            } else if self.random_number < num {
                Err(" -> your number is greater than chosen number ...")
            } else {
                Ok(" -> Good job, you wind the game ...")
            }
        }
    }

    pub fn get_user_number() -> Result<u32, Box<dyn Error>> {
        print!(" -> enter a number: ");
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err)),
        }

        let mut buffer: String = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err)),
        }

        match buffer.trim().parse::<u32>() {
            Ok(resultant) => Ok(resultant),
            Err(err) => Err(Box::new(err)),
        }
    }
}
