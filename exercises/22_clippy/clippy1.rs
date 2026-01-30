// by [apalala@gmail.com](https://github.com/apalala)
// by Gemini (2026-01-29)

use std::f32::consts::PI;

fn main() {
    // FIX: Use the built-in constant and fix the parenthesis
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
