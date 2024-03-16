fn main() {
    println!("Hello, world!");
}
//     //compound types
//     // let tuple: (i32, f64, &str) = (45, 6.78, "Hello");
//     // let array: [u8; 3] = [10,20,30];

//     //custom data types
//     // struct User {
//     //     active: bool,
//     //     username: String,
//     //     email: String,
//     //     sign_in_count: u64,
//     //     mobileno: u32
//     // }
    
//     let sudham: (i32, f64, &str) = (45, 6.78, "Hello");
//     println!("name of user1 ------{:#?}",sudham.0);

//     #[derive(Debug)]
//     struct AadaarUser {
//         username: String,
//         email: String,
//         mobileno: u64
//     }

//     let sudham = AadaarUser {
//         username: String::from("sudham_123"),
//         email: String::from("someone@example.com"),
//         mobileno: 9372288275
//     };

//     let monu = AadaarUser {
//         username: String::from("hero123"),
//         email: String::from("someone@example.com"),
//         mobileno: 9009009990
//     };

//     println!("{:#?}",sudham);
//     println!("name of user ------{:#?}",sudham.username);
// }
///////////////////////////////////////////////////////////////////////

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let x = build_user(String::from("sudhamsingh2412@gmail.com"), String::from("sudham"));
//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("monu"),
//         ..x
//     };
//     println!("user info {:#?}",user2);
//     println!("user info {:#?}",x);
// }
// fn build_user(email: String, username: String) -> User {

//     // let x = User {
//     //     active: true,
//     //     username: username,
//     //     email: email,
//     //     sign_in_count: 1
//     // };

//     // x

//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1
//     }
// }
    

//////////////////////////////////////////////////////////////////
//Using Tuple Structs Without Named Fields to Create Different Types
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }