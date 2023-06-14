#[derive(Debug)]
struct Rectangle {
    length: u64,
    width: u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        return self.length * self.width;
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.length >= other.length;
    }

    fn square(size: u64) -> Self {
        return Self {
            length: size,
            width: size
        };
    }
}

fn main() {

    let rectangle = Rectangle {
        length: 300,
        width: 100
    };

    let rec2 = Rectangle {
        length: 200,
        width: 50
    };




    println!("{:?}", Rectangle::square(5));
}
