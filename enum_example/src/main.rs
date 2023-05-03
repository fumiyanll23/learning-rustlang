#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn show_this_message(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let mut message = Message::Quit;
    message.show_this_message();
    message = Message::Move { x: 30, y: 40 };
    message.show_this_message();
    message = Message::Write("content".to_string());
    message.show_this_message();
    message = Message::ChangeColor(255, 0, 0);
    message.show_this_message();
}
