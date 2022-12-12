#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    // * Create an instance of that struct
    let mut user1 = User {
        active: true,
        username: String::from("Jason, Wang"),
        email: String::from("wanghung09@gmail.com"),
        sign_in_count: 1,
    };

    // * revise user1's email
    user1.email = String::from("wanghung07@gmail.com");

    // * struct update syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("jasonwanghung07@gmail.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // @ equals to ...
    let user2 = User {
        email: String::from("jasonwanghung07@gmail.com"),
        ..user1
    };
    // ! We can no longer use user1's active, usuername and sign_in_count, cuz the data was moved into user2.

    // * Tuple
    let color: (i32, i32, i32) = (0, 0, 0);
    // * Tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // * Uint-Like Struct
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

// * username and email were using -field init shorthand syntax-
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
