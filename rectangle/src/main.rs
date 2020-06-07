#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        (self.length >= r.length) && (self.width >= r.width)
    }
}

fn main() {
    let rectangle = Rectangle {
        length: 50,
        width: 30,
    };
    let rectangle2 = Rectangle {
        length: 50,
        width: 31,
    };
    let rectangle3 = Rectangle::square(30);

    println!("The first rectangle: {:?}", rectangle);
    println!("The area of the rectangle is: {}", rectangle.area());
    println!(
        "Can Rectangle 1 hold Rectangle 2?: {}",
        can_hold(&rectangle, &rectangle2),
    );
    println!(
        "Can Rectangle 1 hold Rectangle 3?: {}",
        can_hold(&rectangle, &rectangle3),
    );
}

fn can_hold(r1: &Rectangle, r2: &Rectangle) -> String {
    if r1.can_hold(r2) {
        String::from("yes")
    } else {
        String::from("no")
    }
}
