fn main() {
    another_function(5, '🥐');
    yet_another_function();
    let b = function_return_value();
    println!{"The value of b is: {}", b};
    let m = plus_one(2);
    println!{"Value of m after adding one: {}", m};
}

fn another_function(x: i32, f: char) {
    println!{"We have {} number of {}", x, f};
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value

fn yet_another_function() {
    let p: i32 = {
        let q = 3;
        q + 7 // Expressions do not include ending semicolons. If you add a semicolon to the end
        // of an expression, you turn it into a statement, which will then not return a value.
    };
    println!{"The value of p is: {}", p};
}

// You can return early from a function by using the return keyword and specifying a value, but most
// functions return the last expression implicitly.

fn function_return_value() -> i32 {
    let a = 10;
    a + 7
}

fn plus_one(k: i32) -> i32 {
    k + 1
}