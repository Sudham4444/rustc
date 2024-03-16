//function

fn main() {
    println!("Hello, world!");
    
    sum_of_x_and_y(30,50);
    sum_of_x_and_y(40,60);
    sum_of_x_and_y(50,70);
    sum_of_x_and_y(60,90);
    another_function();
    another_function1(5);
    print_labeled_measurement(5, 'h');
}
/////////////////////////////////////////////////////////////////////////////////////////
//sum function
fn sum_of_x_and_y(x: u32 , y: u32) {
    let sum = x + y;
    println!("sum of x & y: {sum}");
}

// another function
fn another_function() {
    println!("Another function.");
}

//function with parameters
fn another_function1(x: i32) {
    println!("The value of x is: {x}");
}

//separate parameters functions
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


//function with retuen values
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();
//     println!("The value of x is: {x}");
// }