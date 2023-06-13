

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/*
fn plus_one_failed(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1)
    }
}
 */

fn main() {
    let int_option: Option<u32> = Some(2);
    let bool_option: Option<bool> = Some(true);
    let none_bool: Option<bool>=None;

    assert_eq!(bool_option.is_some(), true);
    assert_eq!(none_bool.is_some(), false);
    let result=5+int_option.unwrap();
    println!("sum is {:?}",result);

    //awesome power of options
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("cool right??  {:?} {:?} ",six,none);
    //show plus_one_failed


}
