/*
 * I am dedicated to my path. I am learning to become a Rust and Zig developer,
 * on small step at a time. I hold onto my dream not as a guarantee of success,
 * but as a commitment to keep trying every single day.
 */

//! Review The Book

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut a = 300;
    let mut b = 500;

    println!(" -> before swap ->> a: {}, b: {} ", a, b);
    my_swap(&mut a, &mut b);
    println!(" -> after  swap ->> a: {}, b: {} ", a, b);

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let ref mut a = 700;
    let ref mut b = 900;

    println!(" -> before swap ->> a: {}, b: {} ", a, b);
    my_swap(a, b);
    println!(" -> after  swap ->> a: {}, b: {} ", a, b);

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let a = &mut 11000;
    let b = &mut 13000;

    println!(" -> before swap ->> a: {}, b: {} ", a, b);
    my_swap(a, b);
    println!(" -> after  swap ->> a: {}, b: {} ", a, b);

    end_message("0.1.0");
    Ok(())
}

fn end_message(version: &str) {
    println!("\n 私の夢, トヨタのプログラマーになりたいです。");
    println!(" この目標を達成するまで、毎日昨日よりも良いコードを書きます。");
    println!(" ラミン あなたは本当に素晴らしい。また会いましょう。{version}\n");
}

fn my_swap<'a>(x: &'a mut i32, y: &'a mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
