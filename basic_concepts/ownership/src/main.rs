fn main() {
    println!("simple _works !!");
    simple_ownership();
    println!("complex_ownership_string  not so!");
    complex_ownership_string();
    println!("complex_ownership_string  that works");
    complex_ownership_string_that_works();

    println!("functions demo");
    functions_demo();
    println!("return demo");
    return_demo();

    println!("refrences_basic");
    refrences_basic();


    println!("failed_to_change");
    failed_to_change();

    println!("success_to_change");
    success_to_change();

    println!("slice and error");
    slice_and_error();

    //TODO show dangle demo
    
}

fn complex_ownership_string_that_works() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
}

fn complex_ownership_string() {
    let s1 = String::from("hello");
    let s2 = s1;
    //TODO this will not work
    println!("{}, world!", s2);
}
fn simple_ownership(){
    let y=5;
    let x=y;
    println!("x {x}")
}

fn functions_demo(){
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("{}",s);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("in takes_ownership {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("in makes_copy {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



fn return_demo() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("gives_ownership {s1}");

    let mut s2 = String::from("hello");     // s2 comes into scope

    s2=takes_and_gives_back(s2);
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    
    //TODO this wil not work
    //println!("takes_and_gives_back original {s2} new {s3}");
    println!("takes_and_gives_back new {s3}");
                                                
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn refrences_basic() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn failed_to_change() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    //TODO this will not work 
    //some_string.push_str(", world");
}


fn success_to_change() {
    let mut s = String::from("hello");

    good_change(&mut s);
    println!("{s}")
}

fn good_change(some_string: &mut String) {
    some_string.push_str(", world");
}


//fn dangle_demo() {
//    let reference_to_nothing = dangle();
//}

//fn dangle() -> &String {
//    let s = String::from("hello");

//    &s
//}


fn slice_and_error(){
    let mut s = String::from("hello world");

    let word_len:usize = first_word_length(&s);
    let word = first_word(&s);


    //TODO enable to cause an error
    //s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}