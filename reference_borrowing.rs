// fn main() {

//     println!("Hello, world!");

//    //references and borrowing 
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

/////////////////////////////////////////////////////////////
// Mutual references
// fn main() {

//     let mut s1 = String::from("hello");
//     let len = calculate_length(&mut s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &mut String) -> usize {
//     s.push_str(" world");
//     s.len()
// }

/////////////////////////////////////////////////////////////////
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}