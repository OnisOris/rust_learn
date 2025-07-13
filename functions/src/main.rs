fn main() {
    println!("Hello, world!");

    another_function(65);
    strange();
    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: u32) {
    println!("Another function. {x}");
}

fn strange() {
    /*
    Выражение и инструкции
    */
    let y = {
        // let y - инструкция, внтури новой обалсти видимости {} выражеие
        let x = 3;
        x + 1 // Это выражение, поэтому не ставится ";", если поставить, то оно не вернется наружу
        // и превратится в инстуркцию
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5 // неявно возвращает выражение 5
    // если поставить ;, то не вернет
}
