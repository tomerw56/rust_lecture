fn main() {
    if_one_line();
    named_loop();
    while_loop();
    for_each();

}

fn if_one_line(){

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

}

fn named_loop(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_each(){
        let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}