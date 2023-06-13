fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // a short word about ownership - 
    /*

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    //since string is not copied- once used as a key or value -field_name or field_value can not be used!!!!!
    map.insert(field_name, field_value);
     */

    //override values
     scores.insert(String::from("Red"), 10);
     scores.insert(String::from("Red"), 25);

    //add only if key not present!
     scores.entry(String::from("Blue")).or_insert(50);
     scores.entry(String::from("Pink")).or_insert(50);
 
     println!("{:?}", scores);


    //updating based on current value
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        //what does this row do? enters a word if not present and returns a handler to the count object
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
