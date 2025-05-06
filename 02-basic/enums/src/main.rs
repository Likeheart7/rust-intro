fn main() {
    let v4 = IPAddr::IPv4(127, 0, 0, 1);
    let v6 = IPAddr::IPv6(String::from("localhost"));
    dbg!(&v4);
}

#[derive(Debug)]
enum IPAddr {
    IPv4(u32, u32, u32, u32),
    IPv6(String),
}