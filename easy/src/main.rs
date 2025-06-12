use std::fs::read_to_string;
use chrono::{Local, Utc};

// struct is more similar to classes in c++
struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

struct Rect {
    width: u32,
    height: u32,
}
// implement on top of struct
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // static functions are not associated with any instance
    fn debug() -> u32 {
        return 1;
    }
}

// --------------------------ENUM----------------------------//

enum Direction {
    
    North,
    South,
    East,
    West
}

/// enum with value
enum Shape {
    Circle(f32),
    Rectange(f32, f32),
}


fn calculate_area(shape: Shape) -> f32 {
    // pattern matching will help use to calculate are of the shape
    let area = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectange(width, height) => width * height,
    };
    area
}

// --------------------------Options/Result----------------------------//

/*
Option and Result are default enums provided by RUST

1. null values in RUST  (Option)
2. Error handling in RUST (Result)

when function want to return null, use options 

enum Option {
    Some,
    None
}
enum Result {
    ok,
    err
}
*/


fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'o' {
            return Some(index as i32);
        }
    }
    return None;
}


fn read_file(){
    let output = read_to_string("g:/Web3/Learning/rustLearning/easy/src/main.rs");
    match output {
        Ok(value) => println!("{}", value),
        Err(error) => println!("error: {}", error),
    }
}

//  ---------------------------- Package Management ---------------------------//

/*
->  cargo add crono
crates are packages in rust 

*/

fn get_current_time(){
    let now = Utc::now();
    println!("{}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("{}", formatted);

    let local = Local::now();
    println!("{}", local);

}


struct Rect1<T>{
    width: T,
    height: T
}

impl<T: std::ops::Mul<Output=T> + Copy> Rect1<T>{
    fn area(&self)-> T{
        self.width*self.height
    }
}


pub fn calculate_length(temp: &String)-> usize
{   
    return temp.len();
}

pub fn append_text(s: &mut String){
    s.insert(0, '3');
}
 


struct User1{
    active: bool,
    name: String
}

fn printstruct(str: User1){
    println!("This is user data: active {}, name {}", str.active, str.name);
}


// println!("{}", fibonacci(6)); // macro ,  dynamic variable -> ({})
fn main() {
    
    let mut str = String::from("Harkirat");
    // lifetime of this mut is from line 120->121
    let ref1 = &mut str;
    ref1.push_str("Singh");
    let ref2 = &str;
    
    println!("{}", ref2);
    let mut user = User1{
        active: true,
        name: String::from("jaikumar")
    };

   

    // one of the existing syntax to define string 
    // let my_string = String::from("Hello, world!");
    // let len = get_str_len(my_string);
    // println!("the length of the string is {}", len);

    // ---------------------------------------------------------//
    // let user = User {
    //     first_name: String::from("John"),
    //     last_name: String::from("Doe"),
    //     age: 30,
    // };
    
    // println!("{}", user.first_name);
    // println!("Hello, world!");

    // ---------------------------------------------------------//
    let rect = Rect {
        width: 10,
        height: 20,
    };
    // error function is not associated with any instance
    // println!("The area of the rectangle is {}", rect.debug());

    // calling on the class itself as static function
    // println!("default function call  {}", Rect::debug());

    // let shape = Shape::Rectange(10.0, 20.0);
    // println!("The area of the shape is {}", calculate_area(shape));
    
    let index = find_first_a(String::from("Hello, world!"));
    
    match index {
        Some(value)=> println!("the first a is at {}", value),
        None => println!("no a found"),
    }

    // read_file();
    get_current_time();
    

}


// fn get_str_len(str: String) -> usize {
//     str.chars().count()  // implicit return statement
// }


// fn is_even(num: u32) -> bool { 
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// 0 1 1 2 3 5 8
// fn fibonacci(num: u32) -> u32 {
//     let mut first = 0;
//     let mut second = 1;
//     if num == 1 {
//         return first;
//     }
//     if num == 2 {
//         return second;
//     }
//     // _i = [1,num-1)
//     for _i in 1..num-1 {
//         let temp = second;
//         second = second + first;
//         first = temp;
//         // println!("{}",second);
//     }
//     return second;
// }



