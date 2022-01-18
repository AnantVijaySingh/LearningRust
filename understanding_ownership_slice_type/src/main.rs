fn main() {
    let mut sentence = String::from("After all this time. Always.");

    first_word_without_slice(&sentence); // In this case the sentence and the returned value for
    // the end of first word are not in sync and can clear to error if the original sentence
    // variable is changed.

    let first_word = first_word_with_slice(&sentence); // We get back a value that is tied
    // to the original string and is in sync. If we try to modify the sentence variable and then use
    // original reference, we will get a compiler error as we would have broken the rules of
    // borrowing: that if we have an immutable reference to something, we cannot also take a
    // mutable reference.

    println!("The first word is: {}", first_word);

    sentence.clear();

    // println!("The first word is: {}", first_word); -> Will throw error if used after
    // sentence.clear a mutable reference, as we break the rule of borrowing.
}

fn first_word_without_slice(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index]; // Return a string slice using the start of the string and the
            // index of the space as the starting and ending indices.
        }
    }

    &s[..]
}
