fn main() {
    let a: i64 = 5;
    let b: u64 = 43;
    println!("a: i64 = {a}, \nb: u64 = {b}");
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: f64 = {x}");
    println!("y: f32 = {y}");

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("-5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    // bool value
    let bool_value = true;
    println!("bool_value = {bool_value}");

    let c = 'z';
    println!("c = {c}");
    let z: char = 'ℤ';
    println!("z = {z}");
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {heart_eyed_cat}");
}
