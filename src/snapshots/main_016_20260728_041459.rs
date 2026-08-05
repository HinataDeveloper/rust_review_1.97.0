//! Using num_traits
//! My function only accept number data-type

use std::{
    error::Error,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use num_traits::Bounded;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    show_number_one(321654987654321987_u128);
    show_number_one(3216549.321654);

    println!("~~~~~~~~~~~~~~~~~~~~~~");

    show_number_two(321654987);
    show_number_two(321654987.321654);

    println!("\nThe End ...");
    Ok(())
}

fn show_number_one<T>(data: T)
where
    T: Add,
    T: Sub,
    T: Mul,
    T: Div,
    T: Copy,
    T: Clone,
    T: Display,
{
    println!(" number is: {}", data);
}

fn show_number_two<T>(data: T)
where
    T: Bounded,
    T: Display,
{
    println!(" number is: {}", data);
}
