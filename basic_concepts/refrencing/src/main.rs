fn main() {
   let mut name = String::from("Matt");
   change_name(&mut name);
   println!("{}",name)
}
fn change_name(s: &mut String) {
   s.push_str(" Bidewell");
}