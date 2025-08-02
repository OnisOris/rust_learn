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

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("popopopopopopopopopopopopopopo");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("OIIA OIIA OIIA OIIA");
}
