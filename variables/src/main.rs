// By default variables in rust are immutable
// Variable can be made mutable using the keyword mut
// While variables are immutable by default they are not the same as constants

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    second_function();
    numeric_operations();
    character_type();
    compound_types();
}

fn second_function() {
    let y = 5;
    let y = y + 2;
    {
        let y = y * 3;
        println!("The value of y in the inner scope is: {}", y)
    }
    println!{"The value of y is: {}", y}
}

fn numeric_operations(){
    // addition
    let sum = 5 + 7;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 56 / 32;
    let remainder = 43 % 5;

    println!{"The sum is {}", sum};
    println!{"The quotient is {}", quotient};
    println!{"The integral quotient is {}", floored};
    println!{"The remainder is {}", remainder};
}

fn character_type(){
    let c = 'z';
    let z = 'ℤ';
    let ice_cream = '🍦';

    println!{"{} {}, and the ice cream emoji {}", c, z, ice_cream};
}

fn compound_types(){
    // Tuple is a general way of grouping together a number of values with a variety of types into one compound type
    let tup = (500, 7.3, '🍫');

    // The tuple without any values, (), is a special type that has only one value, also written ().
    // The type is called the unit type and the value is called the unit value. Expressions
    // implicitly return the unit value if they don’t return any other value.

    // To get the individual values out of a tuple, we can use pattern matching to destructure a
    // tuple value
    let (_x, _y, z) = tup;
    println!{"The value of z is: {}", z};

    // In addition to destructuring through pattern matching, we can access a tuple element directly
    // by using a period (.) followed by the index of the value we want to access.
    let _five_hundred = tup.0;
    let _seven_point_three = tup.1;
    let chocolate_bar = tup.2;

    println!{"The value of chocolate_bar is: {}", chocolate_bar};

    // Unlike a tuple, every element of an array must have the same type. Arrays in Rust are
    // different from arrays in some other languages because arrays in Rust have a fixed length,
    // like tuples.

    let _arr = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // if you want to create an array that contains the same value for each element, you can specify
    // the initial value, followed by a semicolon, and then the length of the array in square brackets

    let ar = [3; 5];

    println!{"System generated array: {}", ar[1]};
    println!{"The second months is: {}", months[1]};
}