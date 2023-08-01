// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255, 0, 128));
}
