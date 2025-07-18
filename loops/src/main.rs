fn main() {
    let mut counter = 0;

    let _output = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };
    println!("_output = {_output} \n counter = {counter}");

    'pupupu: loop {
        println!("push me");
        loop {
            counter -= 1;
            println!("attach me, counter = {counter}");
            if counter == 0 {
                break 'pupupu;
            }
        }
    }
}
