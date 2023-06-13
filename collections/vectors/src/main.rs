fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut v2: Vec<i32> = Vec::new();
    v2.push(5);
    //will not work
    //v2.push(6.0);

    //cool demo on the power of option

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(15);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no 15 element."),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];

    let first = &v4[0];
    //will not work 
    //v.push(6);

    println!("The first element is: {first}");
    
}
