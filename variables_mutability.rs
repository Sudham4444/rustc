// fn main() {
//     println!("Hello, world!");
// }

fn main() {

    // variable and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    //println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {x}");


    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("the values of const is {THREE_HOURS_IN_SECONDS}");


    //shadowing
        let x = 5;
    
        let x = x + 1;
    
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
    
        println!("The value of x is: {x}");

}