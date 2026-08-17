// Date: Thu Aug 17 2026

// Project: Review The Book Rust Programming Language
// Goal: Using tuple struct
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

use std::fmt::Display;

fn main() {
    println!("\n");

    let mut my_color = Color::build();
    my_color.set_first(102);
    my_color.set_second(96);
    my_color.set_third(21);

    println!("value of my_color is: {}", my_color);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let this_color = Color::build_from(120, 131, 22);
    println!("value of this color is: {}", this_color);

    println!("\nThe End ...\n");
}

struct Color(i32, i32, i32);

impl Color {
    fn build() -> Color {
        Self(0, 0, 0)
    }

    fn build_from(x: i32, y: i32, z: i32) -> Color {
        Color(x, y, z)
    }

    fn set_first(&mut self, first: i32) {
        self.0 = first;
    }

    fn set_second(&mut self, second: i32) {
        self.1 = second;
    }

    fn set_third(&mut self, third: i32) {
        self.2 = third;
    }

    fn get_first(&self) -> i32 {
        self.0
    }

    fn get_second(&self) -> i32 {
        self.1
    }

    fn get_third(&self) -> i32 {
        self.2
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.get_first(),
            self.get_second(),
            self.get_third()
        )
    }
}
