struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("ex@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "active: {}, email: {}, username: {}, sign in no. {}",
        user1.active, user1.email, user1.username, user1.sign_in_count
    ); // -> active: true, email: ex@example.com, username: username123, sign in no. 1

    user1.email = String::from("newemail@example.com");

    println!("new email: {}", user1.email); // -> new email: newemail@example.com

    let user2 = build_user(String::from("usr2@example.com"), String::from("user2"));

    println!("user: {}, email: {}", user2.username, user2.email); // -> user: user2, email: usr2@example.com
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // shorthand field populate, due to the field is the same name as the parameter
        username, // shorthand field populate, due to the field is the same name as the parameter
        active: true,
        sign_in_count: 1,
    }
}
