/*
 * I am dedicated to my path. I am learning to become a Rust and Zig developer,
 * on small step at a time. I hold onto my dream not as a guarantee of success,
 * but as a commitment to keep trying every single day.
 */

//! Review The Book

use crate::guess_game::{Database, RangeError, get_user_number};
use std::{cmp::Ordering, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut us_num: u32;
    let mut guess_game = Database::new(0..=100);

    'guess_game: loop {
        'user_number: loop {
            match get_user_number(0..=100_u32) {
                Ok(num) => {
                    us_num = num;
                    break 'user_number;
                }
                Err(err) => println!(
                    " -> error: {}",
                    err.downcast_ref::<RangeError>().unwrap().get_message()
                ),
            }
        }

        match guess_game.compare(us_num) {
            Ordering::Equal => {
                println!(" -> Good job, you won the game ...");
                break 'guess_game;
            }
            Ordering::Greater => println!(" -> Your number is greater ..."),
            Ordering::Less => println!(" -> Your number is less ..."),
        }
    }

    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    for item in guess_game.get_user_numbers() {
        print!(" {}", item);
    }
    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    end_message("0.1.0");
    Ok(())
}

fn end_message(version: &str) {
    println!("\n 私の夢, トヨタのプログラマーになりたいです。");
    println!(" この目標を達成するまで、毎日昨日よりも良いコードを書きます。");
    println!(" ラミン あなたは本当に素晴らしい。また会いましょう。{version}\n");
}

mod guess_game {
    use std::{
        cmp::Ordering,
        error::Error,
        fmt::{Debug, Display},
        io::Write,
        ops::{Bound, RangeBounds},
    };

    use rand::{RngExt, distr::uniform::SampleRange};

    pub struct RangeError {
        min_range: u32,
        max_range: u32,
        message: String,
    }

    impl Debug for RangeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RangeError")
                .field("min_range", &self.min_range)
                .field("max_range", &self.max_range)
                .field("message", &self.message)
                .finish()
        }
    }

    impl Display for RangeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "[min_range: {}, max_range: {}, message: {}]",
                self.min_range, self.max_range, self.message
            )
        }
    }

    impl Error for RangeError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }

        fn cause(&self) -> Option<&dyn Error> {
            self.source()
        }
    }

    impl RangeError {
        pub fn create(min_range: u32, max_range: u32, message: &str) -> RangeError {
            RangeError {
                min_range,
                max_range,
                message: message.to_string(),
            }
        }

        pub fn get_min(&self) -> u32 {
            self.min_range
        }

        pub fn get_max(&self) -> u32 {
            self.max_range
        }

        pub fn get_message(&self) -> &str {
            &self.message.as_str()
        }
    }

    pub struct Database {
        random_number: u32,
        user_numbers: Vec<u32>,
    }

    impl Database {
        pub fn new<I>(range: I) -> Database
        where
            I: SampleRange<u32>,
        {
            Database {
                random_number: rand::rng().random_range(range),
                user_numbers: Vec::new(),
            }
        }

        pub fn compare(&mut self, user_number: u32) -> Ordering {
            self.user_numbers.push(user_number);
            user_number.cmp(&self.random_number)
        }

        pub fn get_user_numbers(&self) -> &[u32] {
            &self.user_numbers
        }
    }

    pub fn get_user_number<R, T>(range: R) -> Result<u32, Box<dyn Error>>
    where
        R: RangeBounds<T>,
        T: Debug + Copy + Into<u32>,
    {
        let start_number: u32;
        let end_number: u32;
        match range.start_bound() {
            Bound::Included(start) => start_number = (*start).into(),
            Bound::Excluded(start) => start_number = (*start).into(),
            Bound::Unbounded => panic!("Error: Unbounded ..."),
        }
        match range.end_bound() {
            Bound::Included(end) => end_number = (*end).into(),
            Bound::Excluded(end) => end_number = (*end).into(),
            Bound::Unbounded => panic!("Error: Unbounded ..."),
        }
        print!(
            " -> enter a number between {} and {}: ",
            start_number, end_number
        );
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let user_number = buffer.trim().parse::<u32>()?;

        if user_number < start_number || user_number > end_number {
            return Err(Box::new(RangeError::create(
                start_number,
                end_number,
                &format!(
                    "user number is out of bound: ({}). It should be between {} and {}",
                    user_number, start_number, end_number
                ),
            )));
        }
        Ok(user_number)
    }
}
