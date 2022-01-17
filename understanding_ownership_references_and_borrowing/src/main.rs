fn main() {
    let mut s1 = String::from("Hello");

    let len = calculate_length(&s1); // Ampersands represent references, and they allow
    // you to refer to some value without taking ownership of it.

    println!{"The length of {} is {}", s1, len};

    // change(&s1); --> Doesn't work because references are immutable by default like variables.

    change_mutable(&mut s1);

    println!("{}", s1);

    // - - - - - - - - - - - - - - - - - - - - - - - - //
    // Mutable references have one big restriction: you can have only one mutable reference to a
    // particular piece of data at a time. We can use curly brackets to create a new scope, allowing
    // for multiple mutable references, just not simultaneous ones

    let mut s2 = String::from("General Kenobi");

    {
        let _r1 = &mut s2;
    }

    let r2 = &mut s2;

    println!("{}", r2);

    // - - - - - - - - - - - - - - - - - - - - - - - - //
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    // This is to prevent one part of the program from changing the data while another part doesn't
    // expect the data to be changed. This is to prevent data race condition.

    let mut s3 = String::from("Do. Or do not. There is no try");

    let y1 = &s3; // no problem
    let y2 = &s3; // no problem
    println!("{} {}", y1, y2); // variables r1 and r2 will not be used after this point, scope ends.
    // Thus now we can pass reference to s3 to r3 as below.

    let r3 = &mut s3;
    println!("{}",r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

/*
// Won't work because references are immutable by default.
fn change(some_string: &String) {
    some_string.push_str(", world!");
}
*/

fn change_mutable(another_string: &mut String) {
    another_string.push_str(" there");
}