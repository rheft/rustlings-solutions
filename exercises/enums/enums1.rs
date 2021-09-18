// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String)
}

fn main() {
    println!("{:?}", Message::Quit(String::from("Quitting")));
    println!("{:?}", Message::Echo(String::from("Echoing")));
    println!("{:?}", Message::Move(String::from("Moving")));
    println!("{:?}", Message::ChangeColor(String::from("changing")));
}
