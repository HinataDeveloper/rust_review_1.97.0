//! Idiomatic & Zero-Dependency Generic Number Printer
//! Target: Rust 2021 Edition

use std::fmt::Display;

/// یک Trait محلی برای نشانه‌گذاری تایپ‌های عددی مدنظر ما.
/// این کار ما را از کریت‌های خارجی بی‌نیاز می‌کند.
pub trait Numeric: Display + Copy {}

// پیاده‌سازی اتوماتیک (Blanket Implementation) برای تمام تایپ‌های عددی استاندارد
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

fn main() {
    // نیازی به بازگرداندن Result در main نیست وقتی خطا رخ نمی‌دهد.
    // استفاده از println! ساده کافی است.
    println!();

    show_number(321_654_987_654_321_987_u128);
    show_number(3_216_549.321_654_f64);

    println!("~~~~~~~~~~~~~~~~~~~~~~");

    show_number(321_654_987_i32);
    show_number(321_654_987.321_654_f64);

    println!("\nThe End ...");
}

/// تابعی با طراحی Idiomatic و مینیمال برای نمایش اعداد.
/// Trait Boundها بسیار خوانا و تمیز شده‌اند.
fn show_number<T>(data: T) 
where 
    T: Numeric 
{
    println!("number is: {}", data);
}
