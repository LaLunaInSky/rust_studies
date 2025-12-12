#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr02 {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be definied here
        
        println!(
            "{:?}",
            self
        );
    }
}

fn main() {
    println!(
        "chapter defining-an-enum/enum_values\n"
    );

    let ip_four = IpAddrKind::V4;
    let ip_six = IpAddrKind::V6;

    route(ip_four);
    route(ip_six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_02 = IpAddr02::V4(
        // String::from("127.0.0.1")
        127, 0, 0, 1
    );
    let loopback_02 = IpAddr02::V6(
        String::from("::1")
    );

    let m = Message::Write(
        String::from("hello")
    );
    m.call();
}

fn route(
    ip_kind: IpAddrKind
) {
    println!(
        "The ip_kind is: {ip_kind:?}"
    );
}