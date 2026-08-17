// Date: Thu Aug 17 2026

// Project: Review The Book Rust Programming Language
// Goal: Using struct
// Dependency: Without dependency

// rustc 1.100.0-nightly (34baba539 2026-08-16)
// binary: rustc
// commit-hash: 34baba5394fcbda4cba7b7c1964a6db421c77c91
// commit-date: 2026-08-16
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.0

// cargo 1.100.0-nightly (8a0d8afba 2026-08-15)
// release: 1.100.0-nightly
// commit-hash: 8a0d8afba810304bcf9a10bac430be80dd470233
// commit-date: 2026-08-15
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Ubuntu 26.4.0 (resolute) [64-bit]
// Kernel Version: 7.0.14-070014-generic
// Firmware Version: 71CN51WW(V1.21)

// rustup 1.29.0 (28d1352db 2026-03-05)

fn main() {
    println!("\n");

    let my_color: Color = Color(120, 200, 101);
    let my_point: Point = Point(200, 96, 41);

    println!("value of my color is: {:?}", my_color);
    println!("value of my point is: {:?}", my_point);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!("my color index 0: {}", my_color.0);
    println!("my color index 1: {}", my_color.1);
    println!("my color index 2: {}", my_color.2);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!("my point index 0: {}", my_point.0);
    println!("my point index 1: {}", my_point.1);
    println!("my point index 2: {}", my_point.2);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!("\nThe End ...\n");
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);
