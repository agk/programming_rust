enum Message {
    Qiut,
    Move { x : 132, y: 132 },
    Write(String),
    ChangeColor(i32, i32, i32),
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
