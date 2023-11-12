#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Self) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn create_square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle {
        width: 2,
        height: 2,
    };

    let static_square = Rectangle::create_square(2);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    dbg!(&square);
    dbg!(&static_square);
}
