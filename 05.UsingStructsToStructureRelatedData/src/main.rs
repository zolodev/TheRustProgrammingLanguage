// Unit struct
struct User {
    active: bool,
    username: &str, // Error, expected named lifetime parameter
    email: &str,    // Error, expected named lifetime parameter
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
