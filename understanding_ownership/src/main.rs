// In Rust, the memory is automatically returned once the variable that owns it goes out of scope.

fn main() {
    // This type manages data allocated on the heap and as such is able to store an amount of text
    // that is unknown to us at compile time. This kind of string can be mutated.
    let mut s = String::from("Hello");

    // This is different from the standard string literal as the one below. That can't be modified.
    // You can however replace the variable with new value.
    let _str_literal = "Hello";
    // String literal: we know the contents at compile time, so the text is hardcoded directly into
    // the final executable. This is why string literals are fast and efficient. Stack vs heap.

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!{"{}", s};

    move_concept();
    clone_concept();
    copy_concept();
}

fn move_concept() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // In this case s2 points to the same memory the heap like s1
    // To prevent double free error, after the line let s2 = s1, Rust considers s1 as no longer
    // valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope
    // s1 was moved into s2, and s1 was invalidated, thus the print macro below will give error.
    // println!("{}, world!", s1);
}

fn clone_concept() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    // This time the print macro works as s1 is no longer invalidated but the value on the heap is
    // copied and a new pointer to this copied data is provided to s2
}

fn copy_concept() {
    let x = 5;
    let y = x;
    // The reason is that types such as integers that have a known size at compile time are stored
    // entirely on the stack, so copies of the actual values are quick to make. That means there’s
    // no reason we would want to prevent x from being valid after we create the variable y. In
    // other words, there’s no difference between deep and shallow copying here, so calling clone
    // wouldn’t do anything different from the usual shallow copying and we can leave it out.
    println!("x = {}, y = {}", x, y);
}