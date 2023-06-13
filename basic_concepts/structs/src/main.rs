

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

    //associated function gl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}


fn main() {
    let rectangle_1: Rectangle= Rectangle{
        width:10,
        height:100
    };


    let rectangle_2: Rectangle= Rectangle {
          width:40,
        ..rectangle_1
    };

    println!("rect1 is {:?}", rectangle_1);
    println!("rect2 is {:?}", rectangle_2);

    println!("rect2 area {:?}", rectangle_2.area());


    println!("rect2 can hold rect 1 {:?}", rectangle_2.can_hold(&rectangle_1));

    let scale = 2;
    let rect_debug = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect_debug);


    let square_from_rect:Rectangle= Rectangle::square(10);
    println!("square from rect_1 {:?} ",square_from_rect);

}