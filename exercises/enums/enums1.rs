// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello world")));
    println!("{:?}", Message::Move { x: 10, y: 30 });
    println!("{:?}", Message::ChangeColor(200, 255, 255));
}
