use std::option::Option;

#[derive(Debug)]
enum IpAddressType {
    Ipv4(u8, u8, u8, u8),
    Ipv6(String)
}

// This will give error as enum Option is already imported using std::option::Option
// enum Option<T> {
//     None,
//     Some(T),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home_ip = IpAddressType::Ipv4(1,0,0,1);
    println!("{:?}", home_ip);

    let some_value : Option<i8> = Some(4);
    println!("{:?}", some_value);

    let coin = Coin::Dime;
    let amount = value_in_cents(&coin);
    println!("Valus is ==> {amount}");

    let config_max : Option<u8> = Some(7);
    // let config_max : Option<u8> = None;
    if let Some(max) = config_max {
        println!("Condition matched");
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // other => 0,
        _ => 0,
    }
}