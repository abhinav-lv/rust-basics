#![allow(dead_code)]
#![allow(unused_variables)]
pub fn enums() {
    let a = Mood::Depressed(3000);
    if let Mood::Depressed(value) = a {
        println!("{value}");
    }

    let m1 = Message::Quit;
    m1.call();
    let m2 = Message::Move { x: 420, y: 69 };
    m2.call();
    let m3 = Message::Write(String::from("to Shreya"));
    m3.call();
    let m4 = Message::ChangeColor(255, 255, 255);
    m4.call();

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let num = Some(5);
    let six = exercise(&num);
    println!("{:?}", six);

    enum_ownership();
}

enum Mood {
    Happy(u32),
    Depressed(u32),
    Psychotic(u32),
    Angry(u32),
    Confused(u32),
}

/* -------------------------- */
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Quit => {
                println!("I quit!!!");
            }
            Self::Move { x, y } => {
                println!("Move to {}, {}", x, y);
            }
            Self::Write(s) => {
                println!("I am writing {}", s);
            }
            Self::ChangeColor(a, b, c) => {
                println!("Change color to {}, {}, {}", a, b, c);
            }
        }
    }
}
/* -------------------------- */

/* -------------------------- */
#[derive(Debug)]
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
/* -------------------------- */

fn exercise(num: &Option<i32>) -> Option<i32> {
    match num {
        Some(a) => Some(a + 1),
        None => None,
    }
}

fn dice_roll() {
    let roll = 9;
    match roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),    // for all other cases (default)
        _ => reroll(),                  // default without a value
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(moves: i32) {}
    fn reroll() {}
}

fn enum_ownership() {
    let opt = Some(String::from("Hello!"));

    // This will compile
    // match opt {
    //     Some(_) => println!("Some"),
    //     None => println!("None!")
    // }  
    // dbg!(opt);

    // But this will not
    // match opt {
    //     Some(s) => println!("{}", s),
    //     None => println!("None"),
    // }  
    // dbg!(opt);

    // To fix:
    match &opt {
        Some(s) => println!("{}", s),
        None => println!("None")
    }
    dbg!(opt);
}
