/*
 * I am dedicated to my path. I am learning to become a Rust and Zig developer,
 * on small step at a time. I hold onto my dream not as a guarantee of success,
 * but as a commitment to keep trying every single day.
 */

//! Review The Book

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let message: &str = "I am dedicated to my path.";
    let mut words: Vec<&'static str> = Vec::new();

    let mut first: usize = 0;
    let mut second: usize;

    for (index, value) in message.chars().enumerate() {
        if value == ' ' {
            second = index;
            words.push(&message[first..second]);
            first = second + 1;
        }
    }

    for item in words {
        println!("{}", item);
    }

    end_message("0.1.0");
    Ok(())
}

fn end_message(version: &str) {
    println!("\n 私の夢, トヨタのプログラマーになりたいです。");
    println!(" この目標を達成するまで、毎日昨日よりも良いコードを書きます。");
    println!(" ラミン あなたは本当に素晴らしい。また会いましょう。{version}\n");
}

// fn my_swap<'a, T>(mut x: &'a T, mut y: &'a T) {
//     let temp = x;
//     x = y;
//     y = temp;
// }
