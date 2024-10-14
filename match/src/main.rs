#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Kentucky,
    Teksas,
    California,
    Virginia,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
// fn value_in_cents2(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("state quarter from {state:?}!");
//             25
//         }
//     }
// }

fn main() {
    let coin = Coin::Quarter(UsState::California);
    let value = value_in_cents(coin);
    println!("The value of the coin is: {} cents", value);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{six:?}");
    println!("{none:?}");
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
