enum IpAddrKind {
V4,
V6,
}

struct {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;
    println!("Hello, world!");
}
