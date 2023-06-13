#[derive(Debug)]
enum IpAddrKind_Simple {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind_Complex {
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

        println!("called {:?}",self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}




fn add_fancy_hat() {
    println!("in add_fancy_hat ")
}
fn remove_fancy_hat() {
    println!("in remove_fancy_hat ")
}

fn main() {
    //simple
    let ip_simple=IpAddrKind_Simple::V4;
    println!("simple {:?}",ip_simple);

    //complex
    let home = IpAddrKind_Complex::V4(127,0,0,1);

     let loopback = IpAddrKind_Complex::V6(String::from("::1"));

    println!("v4 {:?} v6 {:?}",home,loopback);

    //message and enum methods
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("match demo 1");
    let coin_instance:Coin=Coin::Quarter(UsState::Alabama);
    println!("value of coin is {:?}",value_in_cents(coin_instance));

    
    //Power of operations
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}
