// this will not work because at same point of time there cannot be some mutable and imutable reference

// fn main() {
//     let mut str = String::from("Jaikumar");
//     let ref1 = &mut str;
//     let ref2 = &str;

//     println!("{} {}", ref1, ref2);
// }

// ------------------------------------------------------------------------

// fn main() {
//     let mut str = String::from("Harkirat");
//     let ref1 = &mut str;
//     ref1.push_str("Singh");
//     let ref2 = &str;

//     println!("{}", ref2);
// }

// pub trait Draw {
//     fn draw(&self) -> String {
//         String::from("Draw")
//     }
// }

// pub trait CalculateArea<T> {
//     fn calculate_area(&self) -> T {}
// }

// struct Circle<T> {
//     radius: T,
// }

// impl<T: Draw + CalculateArea<T>> Circle<T> {
//     fn display() {
//         const str1 = String::from("Circle");
//         const str2: &String = &str1;

//     }
// // }

// struct User {
//     active: bool,
//     sign_in_count: u64,
// }

use chrono::{Local, Utc};

macro_rules! say_hello {
    ($arg:expr) => {
        println!("Hello, world {}", $arg);
    };
}

fn main() {
    // let mut user1 = User {
    //     active: true,
    //     sign_in_count: 1,
    // };

    // print_name(user1);
    // print!("User 1 username: {}", user1.active); // Error - can not use borrowed value

    // let current_local_time = Local::now();
    // println!("Current time: {}", current_local_time);

    // let current_time = Utc::now();
    // println!("Current time: {}", current_time);

    say_hello!("jaikumar");
}

// fn print_name(user1: User) {
//     print!("User 1 username: {}", user1.active);
// }
