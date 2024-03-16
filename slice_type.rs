fn main() {

    let mut my_string = String::from("hello world");
    // let x = first_word(&my_string);

    // let mut s = String::from("hello world");

    let x = first_word(&my_string); // x will get the value 5
    my_string.clear();

    println!("first word of my string upto {x}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // println!("{:?}",bytes);
    // println!("{}",b' ');

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/////////////////////////////////////////////////////////
//string slice
// let s = String::from("hello world");

// let hello = &s[0..5];
// let world = &s[6..11];
