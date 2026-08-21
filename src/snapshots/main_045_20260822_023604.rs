// Date: Thu Aug 22 2026

// Project: Review The Book Rust Programming Language
// Goal: How to use HashMap: Check if key exist.
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

    let message = String::from(
        "Rust is a very fast and relible programming language and is very efficient for memory safty.",
    );

    let mut analyze = HashMap::new();

    for item in message.split_whitespace() {
        println!("{}", item);
        let counter = analyze.entry(item).or_insert(0);
        *counter += 1;
    }

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    for (key, value) in &analyze {
        println!("Key:{}, Value:{}", key, value);
    }

    println!("\nThe End ...\n");
}
