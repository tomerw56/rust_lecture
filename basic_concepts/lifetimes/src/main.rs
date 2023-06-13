/*

/*
this wil not work due to life time
 Rust can’t tell whether the reference being returned refers to x or y. Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y!
  */
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
 */


//this works
//string1 lives in the outer scope
//string2 and result are used and live in inner scope 
//this means they all ive together -right?
fn working_sample(){//outer scope
    let string1 = String::from("long string is long");

    {//inner scope
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    
}



//this does not works
//string1 and result live and used  tll the outer scope
//string2  is used and live in inner scope 
//this means we use results after string2 is dead -right?
/*fn none_working_sample(){//outer scope
    let string1 = String::from("long string is long");
    let result;
    {//inner scope
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}*/

//as long as X ad Y are alive the return value is valid as well
//see https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    working_sample();
}
