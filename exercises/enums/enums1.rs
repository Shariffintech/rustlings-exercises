// enums1.rs
// No hints this time! ;

#[derive(Debug)]
enum Message {
    Quit,
    Move,
    Echo,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
