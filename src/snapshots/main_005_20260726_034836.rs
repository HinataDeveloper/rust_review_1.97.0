/*
 * I am dedicated to my path. I am learning to become a Rust and Zig developer,
 * on small step at a time. I hold onto my dream not as a guarantee of success,
 * but as a commitment to keep trying every single day.
 */

//! Separate a statement to its words.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let statement: String = String::from("I am dedicated to my path.");
    
    let words = statement.split_whitespace();

    for item in words {
        println!(" {}", item);
    }


    end_message("0.1.0");
    Ok(())
}

fn end_message(version: &str) {
    println!("\n 私の夢, トヨタのプログラマーになりたいです。");
    println!(" この目標を達成するまで、毎日昨日よりも良いコードを書きます。");
    println!(" ラミン あなたは本当に素晴らしい。また会いましょう。{version}\n");
}
