/*
#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IPAddrKind::V4(127,0,0,1);
    let six = IPAddrKind::V6(String::from("::1"));

    route(four);
    route(six);
}

fn route(ip: IPAddrKind) {
    println!("{:?}", ip);
}
*/


/*
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn write_message(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 20, y: 100 };
    let write_message = Message::Write(String::from("writing stuff"));
    let change_color_message = Message::ChangeColor(255, 255, 255);

    quit_message.write_message();
    move_message.write_message();
    write_message.write_message();
    change_color_message.write_message();
}
*/

/*
#[derive(Debug)]
enum UsState {
    Alaska,
    Washington,
    Hawaii,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn main() {
    let quarter = Coin::Quarter(UsState::Hawaii);
    let value = value_of_coin(quarter);

    println!("hi quarter value is: {value}");


}
*/


fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    if let Some(5) = five {
        println!("five");
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None =>  None,
        Some(i) => Some(i + 1),
    }
}