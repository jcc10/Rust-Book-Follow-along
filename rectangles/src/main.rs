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
        if self.width < other.width{
            return false
        }
        if self.height < other.height {
            return false
        }
        return true
    }

    // the one from the book.
    //fn can_hold(&self, other: &Rectangle) -> bool {
    //    self.width > other.width && self.height > other.height
    //}

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    main1();
    main2();
    main3();
}


fn main1() {
    let scale = 1;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn main2() {
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn main3() {

    let scale = 1;
    let rect1 = Rectangle::square(10);

    println!("rect1 is {:?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}