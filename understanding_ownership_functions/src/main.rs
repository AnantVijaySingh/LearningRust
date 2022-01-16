fn main() {
    let s = String::from("Hello"); // s comes into scope.

    takes_ownership(s); // s value moves into function
                                   //  . . .  and is no longer valid here.
    // println!("{}", s); --> will throw an error

    let x = 5; // x comes into scope.

    makes_copy(x);  // x would move into function
                                // but i32 is a copy so it is still okay to use x afterwards.

    println!("{}", x); // x can be used

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - //
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - //
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - //

    let s1 = gives_ownership(); // gives ownership, moves return value to s1

    let s2 = String::from("Aloha"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back
                                                      // which also moves its return value into s3

    println!("Printing s1: {}", s1);   // --> works as expected
    // println!("printing s1 {}", s2); --> throws error as s2 is moved out
    println!("Printing s3: {}", s3);  // --> works as expected

}
// x goes out of scope and then s, but because s was moved, nothing special happens here.
// s1 and s3 go out of scope and are dropped. s2 was moved so nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // some_String goes out of scope, drop is called to free memory from heap.

fn makes_copy(some_integer: i32) { // some_integer is comes into scope.
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // give_ownership will move its return value into the function that calls it

    let another_string = String::from("Chocolate"); // another_string comes into scope

    another_string  // another_string is returned and moves out to the calling function
}

fn takes_and_gives_back(mut a_string: String) -> String { // a_string comes into scope
    a_string.push_str(", world!");

    a_string // a_string is returned and moves out to the calling function
}
