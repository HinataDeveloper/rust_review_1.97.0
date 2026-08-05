//! ...

use std::fmt::Display;

fn main() {
    println!("\n");

    show_number(321_654_987_321_654_987_u128);
    show_number(999_555_.321_654_987);
    // show_number("");

    println!("\nThe End ...");
}

trait Numeric: Display + Copy {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for isize {}

impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for usize {}

impl Numeric for f32 {}
impl Numeric for f64 {}

fn show_number<T: Numeric>(data: T) {
    println!("Number is: {}", data);
}
