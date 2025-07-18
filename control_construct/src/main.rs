use std::io;
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!("Write your age: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("somthing wrong");
    let number_for_condition: usize = input.trim().parse().expect("Your ages very strange");
    if number_for_condition < 18 {
        println!("You won't to drink beer");
    } else {
        println!("You'll to drink beer");
    }
    if number == 3 {
        println!("pupupupupupupu")
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let cond = true;
    let ifcond = if cond { 5 } else { 6 };
    println!("ifcond = {ifcond}");
}
