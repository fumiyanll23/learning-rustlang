#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    let green = Color::Green;
    let yellow = Color::Yellow;
    println!("Green Color Code: {}", color_to_str(&green));
    println!("Yellow Color Code: {}", color_to_str(&yellow));
    find_maybe_number(Some(5));
    find_maybe_number(None);
    let maybe_number: Option<u32> = None;
    if let Some(number) = maybe_number {
        println!("number: {number}");
    } else {
        println!("this is else stmt");
    }
}

fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("found {number}"),
        None => println!("Nothing found."),
    }
}

fn color_to_str(color: &Color) -> &str {
    // Red #FF0000
    // Green #00FF00
    // Blue #0000FF
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
        Color::Yellow => "#FFFF00",
    }
}
