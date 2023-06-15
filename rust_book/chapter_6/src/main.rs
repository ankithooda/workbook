#[derive(Debug)]
enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: u32, y: u32},
    Write(String),
    ChangeColor(u32, u32, u32)
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<USState>),
    Dollar
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    California
}

fn main() {

    let c = Coin::Nickel;
    let q = Coin::Quarter(Some(USState::California));
    let q1 = Coin::Quarter(None);
    println!("{:?} has {} cents.", q, value_in_cents(&q));
    println!("{:?} has {} cents.", q1, value_in_cents(&q1));
}

fn value_in_cents(c: &Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                None => {
                    println!("No State Found.");
                    25
                },                
                USState => {
                    println!("State of the Quarter is {:?}", state);
                    25
                }
            }
        },
        Coin::Dollar => 100
    }
}

fn enum_expr() {
    let _home = IPAddress::V4(127,0,0,1);

    let _loopback = IPAddress::V6(String::from("::1"));

    let m1 = Message::Quit;
    let m2 = Message::Move {
        x: 1,
        y: 2
    };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(1, 2, 3);

    let some_number = Some(5);
    let absent_number: Option<u64> = None;

    println!("{:?}, {:?}, {:?}, {:?}", m1, m2, m3, m4);
    println!("{:?}, {:?}", some_number, absent_number);
}