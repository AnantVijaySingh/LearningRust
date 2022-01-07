// By default variables in rust are immutable
// Variable can be made mutable using the keyword mut
// While variables are immutable by default they are not the same as constants

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    second_function();
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