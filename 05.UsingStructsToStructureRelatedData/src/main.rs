struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("ex@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "active: {}, email: {}, username: {}, sign in no. {}",
        user1.active, user1.email, user1.username, user1.sign_in_count
    ); // -> active: true, email: ex@example.com, username: username123, sign in no. 1
}
