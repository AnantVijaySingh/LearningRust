fn main() {
    let number = 6;

    if number % 4 == 0 {
        print("Number is divisible by 4");
    } else if number % 3 == 0 {
        print("Number is divisible by 3");
    } else if number % 2 == 0 {
        print("Number is divisible by 2")
    } else {
        print("Number is not divisible by 4, 3 or 2");
    }

    // Using if in a let Statement
    second();
}

fn print(text: &str) {
    println!{"{}", text}
}

fn second() {
    let condition = true;
    // Because if is an expression, we can use it on the right side of a let statement
    let number = if condition { 3 } else { 7 };
    println!("The value of number is: {}", number);
}