pub enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    pub fn name(&self) -> String {
        match &self {
            UsState::Alabama => String::from("Alabama"),
            UsState::Alaska => String::from("Alaska"),
        }
    }
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    pub fn value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Coin::Penny => String::from("penny"),
            Coin::Nickel => String::from("nickel"),
            Coin::Dime => String::from("dime"),
            Coin::Quarter(state) => {
                String::from(format!("quarter ({})", state.name()))
            },
        }
    }
}

fn main() {
    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska)
    ];

    for coin in coins.iter() {
        println!(
            "The value of a {} is: {} cent{}",
            coin.name(),
            coin.value(),
            match coin.value() {
                1 => "",
                _ => "s",
            }
        );
    }
}
