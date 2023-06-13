fn main() {
    //const  can not evalualate at run time
    const NUMBER_OF_SECONDS:u32=60;
    const OTHER_CONST:u32 = get_number_of_seconds(); 
    let x = 5;
    //let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn get_number_of_seconds()->u32{
    return 60
}