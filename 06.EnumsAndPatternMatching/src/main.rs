enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    // No need for a struct, we can put data directly into each enum variant
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}
