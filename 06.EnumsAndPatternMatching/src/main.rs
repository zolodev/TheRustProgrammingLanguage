enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Functions can take an enum as a parameter
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

// Functions can take an enum as a parameter
fn route(ip_kind: IpAddrKind) {}
