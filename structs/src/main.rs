fn main() {

    // Instance of a struct. Immutable by default.
    let mut user1 = User {
        email: String::from("Potter@Hogwarts.com"),
        username: String::from("Harry Potter"),
        active: true,
        sign_in_count: 1,
    };
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable

    user1.email = String::from("PotterHarry@Hogwarts.com");

    let user2 = build_user(String::from("GrangerHermione@Hogwarts.com"), String::from("Hermione Granger"));

    let user5 = build_user_shorthand_syntax(String::from("LovegoodLuna@Hogwarts.com"), String::from("Lune Lovegood"));

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("PotterJames@Hogwarts"),
        sign_in_count: user1.sign_in_count,
    };

    //Shorthand syntax
    let user4 = User {
        email: String::from("WeasleyRon@Hogwarts.com"),
        ..user2
    };
    // In this example, we can no longer use user1 after creating user2 because the String in the
    // username field of user1 was moved into user2. If we had given user2 new String values for
    // both email and username, and thus only used the active and sign_in_count values from user1,
    // then user1 would still be valid after creating user2. The types of active and sign_in_count
    // are types that implement the Copy trait so are not bound by the rules of ownership.

    // tuple structs
    let _color1 = Color(255, 255, 0);
    let _point1 = Point(1,0,1);

    // unit-like structs
    let _subject1 = AlwaysEqual;


    // print statements
    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{}", user3.email);
    println!("{}", user4.email);
    println!("{}", user5.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// function that returns an instance of User.
fn build_user (email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Shorthand to avoid reusing the variable names. Because the parameter names and the struct field
// names are exactly the same, we can use the field init shorthand syntax.

fn build_user_shorthand_syntax (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// unit-like structs: structs that don’t have any fields
struct AlwaysEqual;


