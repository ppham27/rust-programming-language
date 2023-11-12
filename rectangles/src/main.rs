#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let tuple1 = (30, 50);

    println!("tuple is {:?}", tuple1);
    println!("tuple is {:#?}", tuple1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    if rect1.width() {
        println!("the rectangle has nonzero width; it's {}", rect1.width);
    }

    dbg!(&rect1);
    println!("rect is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(9);

    println!("I am square: {:?}", square);
}
