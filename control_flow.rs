
//control flow
fn main() {
 
    ////////////////////basic example
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } 
    else {
        println!("condition was false");
    }


    ///////////////////////controlling value of variable
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    ///////////////////////multiple if else
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    ///////////////////LOOP
    ///////////////////break condition (manage)
    use std ::{thread, time};
    let mut x = 0;
    loop {
        println!("Sudham");
        x = x+1;
        println!("value of x == {x}");

        if x >= 5{
            break;
        }

        thread::sleep(time::Duration::from_secs(1));
    }


    ///////////////////WHILE LOOP
    let mut x = 0;
    while x <= 5 {
        x = x+1;
        println!("Sudham");
        println!("value of x == {x}");

        thread::sleep(time::Duration::from_secs(1));
    }


    ////////////////////looping through collection(array)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


    ////////////////////FOR loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    } 
}