use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct just_another_struct{
    value:i32
}

fn main() {
    let int_pair:Pair<i32>=Pair { x: 12, y: 13 };
    int_pair.cmp_display();
    

    let just_another_struct_pair:Pair<just_another_struct>=Pair { x: just_another_struct { value: 12 }, y: just_another_struct { value:13 } };
    //this wil not work as just_another_struct does not impliment what we need!
    //just_another_struct_pair.cmp_display();



}
