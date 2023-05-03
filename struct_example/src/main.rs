#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(width: u32) -> Self {
        Self {
            width,
            height: width,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(60);

    println!(
        "The area of the rectangle is {} square pixels.",
        square.area()
    );

    println!("rect: {:?}", &rect);
    println!("square: {:?}", &square);

    rect.set_width(40);
    println!("rect: {:?}", rect);
}
