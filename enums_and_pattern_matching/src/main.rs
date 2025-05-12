// @note enum declarations are NOT followed by a semicolon
#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

// we can add values to each enum variant instead of having to create a struct having the enum and
// a value separately
#[derive(Debug)]
enum IPAddrKindAndAddr {
    V4(String),
    V6(String),
}

// each variant can have different types too
#[derive(Debug)]
enum IPAddrDiffTypes {
    V4(u8,u8,u8,u8),
    V6(String),
}

// enums can have methods and associated functions too 
impl IPAddrKindAndAddr {
    fn call(&self) {
        // your method stuff here
    }
}

// IpAddr and Option are enums defined in the standard library
// @note There is no null value in Rust!
fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);

    let home = IPAddrKindAndAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddrKindAndAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    
    let home = IPAddrDiffTypes::V4(127,0,0,1);
    let loopback = IPAddrDiffTypes::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    // Option struct with Some and None
    let some_num = Some(5);
    let some_char = Some('e');

    // Since there is no null in Rust, we use the Option enum's None value
    // We have to specify the datatype (here, i32) as Rust cannot infer the type the corresponding Some
    // variant will hold from None
    let absent_num: Option<i32> = None;

    let name: Option<String> = Some("Alice".to_string());
    
    if let Some(n) = name {
        println!("{:?}", n);
    } else {
        println!("No name");
    }

    let x = which_coin(Coin::Quarter);
    println!("{x}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

// match control flow construct
fn which_coin(coin: Coin) -> u8 {
    // in match, suppose it's checking a normal integer value,
    // we use _ placeholder for other cases, kinda like default keyword in switch
    // that is, IF we don't want to use any other value, if we want to use, give it any identifier
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // if we're dealing with Option<String> or any movable value, we should use &x instead of x so
    // that the ownership is not transferred to the match var
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}
