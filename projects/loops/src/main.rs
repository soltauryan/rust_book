fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);

    labeled_loops();
    while_loop();
    loop_collection();
    range();
}


fn labeled_loops() {
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
}


fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


fn loop_collection() {
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("the value is {}", element);
    }
}

fn range() {
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("Liftoff!")
}