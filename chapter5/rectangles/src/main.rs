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
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let res1 = rect1.can_hold(&rect2);
    let res2 = rect1.can_hold(&rect3);

    let area1 = rect1.area();
    let area2 = rect2.area();

    println!("Area 1: {area1}");
    println!("Area 2: {area2}");

    println!("Can rect1 hold rect2?: {res1}");
    println!("Can rect1 hold rect3?: {res2}")
}
