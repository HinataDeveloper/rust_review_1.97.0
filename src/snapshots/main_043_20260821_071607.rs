// Date: Thu Aug 21 2026

// Project: Review The Book Rust Programming Language
// Goal: How to use HashMap : Remove Data
// Dependency: Without dependency

// rustc 1.100.0-nightly (8925ea358 2026-08-20)
// binary: rustc
// commit-hash: 8925ea358a0f265ca61026aadc7ecc506c545cbe
// commit-date: 2026-08-20
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.0

// cargo 1.100.0-nightly (514c56dd7 2026-08-19)
// release: 1.100.0-nightly
// commit-hash: 514c56dd7321eecbfdcf9b6479519cf4edfab906
// commit-date: 2026-08-19
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Ubuntu 26.4.0 (resolute) [64-bit]
// Kernel Version: 7.0.0-30-generic
// Firmware Version: 71CN51WW(V1.21)

// rustup 1.29.0 (28d1352db 2026-03-05)

use std::collections::HashMap;

fn main() {
    println!("\n");

    let mut raphael: HashMap<String, i32> = HashMap::new();

    raphael.insert(String::from("Albert"), 321);
    raphael.insert(String::from("Robert"), 973);
    raphael.insert(String::from("Daniel"), 1023);
    raphael.insert(String::from("Samuel"), 7355);

    for (key, value) in &raphael {
        println!("Key:{}, Value:{}", key, value);
    }

    println!("removeing Robert ...");
    raphael.remove(&String::from("Robert"));

    for (key, value) in &raphael {
        println!("Key:{}, Value:{}", key, value);
    }

    println!("\nThe End ...\n");
}
