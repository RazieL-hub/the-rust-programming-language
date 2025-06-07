enum IpAddrEnum {
V4(u8, u8, u8, u8),
V6(String),
}
/* 
enum Option<T> {
Some(T),
None,
}
*/

enum IpAddrKind {
V4,
V6,
}

#[derive(Debug)] // чтобы проверить штат сразу
enum UsState {
Alabama,
Alaska,
}

enum Coin {
Penny,
Nickel,
Dime,
Quarter(UsState),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
Quit, //not have related structures
Move { x: i32, y: i32 }, //anonym structure
Write(String),
ChangeColor(i32, i32, i32),
}

fn main() {
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;
    println!("Hello, world!");
    let local = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("localhost")
    };
    let looppack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let looppack_v2 = IpAddrEnum::V6(String::from("::2"));

    let some_number = Some(5);
    let sum_string = Some("string literal");
    let absent_number: Option<i32> = None;
    println!("{}", sum_string.is_some());
    println!("{:?}", plus_one(Some(5)));

    let coin = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", coin);
    let some_u8_value = 0u8;
    match some_u8_value {
    0 => println!("ноль"),
    1 => println!("один"),
    3 => println!("три"),
    5 => println!("пять"),
    7 => println!("семь"),
    _ => (),
}
    println!("{}", some_u8_value);
    
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => { 
        println!("State is {:?}", state);
        25
},

}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
}
}

