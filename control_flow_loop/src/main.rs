fn main() {
    labeled_loops();
    return_value_from_loop();
    while_loop();
    loop_array();
    for_loop();
    for_range_loop();
}

fn labeled_loops() {
    let mut count = 0;
    // loop label to help specify a specific loop for break and continue
    'counting_up: loop {
        println!{"count = {}", count}
        let mut remaining = 10;

        loop {
            println!{"remaining = {}", remaining};
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
    println!("End count = {}", count);
}


fn return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter^2;
        }
    };

    println!{"The result is {}", result};
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!{"In {}", number};
        number -= 1;
    }
    println!{"Liftoff!"}
}

fn loop_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = a.len();
    println!{"{}", index};

    while index > 0 {
        println!{"the value is {}", a[index - 1]};
        index -= 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40 ,50];
    for element in a {
        println!{"the value is: {}", element};
    }
}

fn for_range_loop() {
    // Range, which is a type provided by the standard library that generates all numbers in
    // sequence starting from one number and ending before another number.
    // .rev() reverses the range

    for number in (1..4).rev() {
        println!{"{}!", number};
    }
    println!{"Liftoff"}
}