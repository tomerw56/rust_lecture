fn simple_demo(){
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
fn collect_demo(){
    let v1: Vec<i32> = vec![1, 2, 3];

    //what would happen if we only use
    //v1.iter().map(|x| x + 1).collect();
    
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();


    for val in v2.iter() {
        println!("Got: {}", val);
    }

    
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
impl  Shoe{
    fn print_contnet(&self){
        println!("{} {}",self.size,self.style);
    }
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


fn closeurs_demo(){
    let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);
        for val in in_my_size.iter() {
            val.print_contnet();
        }
    

}


fn main() {
    simple_demo();
    println!("collect demo");
    collect_demo();
    println!("closeurs demo");
    closeurs_demo();
    
}
