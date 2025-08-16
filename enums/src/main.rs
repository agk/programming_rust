enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr::V4(127.0.0.1);
let loopback = IpAddr::V6(String::from("::1"));

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
}

fn route(ip_kind: IpAddrKind) {

}
