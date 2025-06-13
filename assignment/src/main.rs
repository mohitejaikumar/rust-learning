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

// use chrono::{Local, Utc};

// macro_rules! say_hello {
//     ($arg:expr) => {
//         println!("Hello, world {}", $arg);
//     };
// }

macro_rules! create_function {
    ($func:ident) => {
        fn $func() {
            println!("Hello, world {}", stringify!($func));
        }
    };
}

create_function!(hello);


use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Person {
    name: String,
    age:u32
}

fn find_element<T: PartialEq>(vec: &Vec<T>, num: T ) -> Option<&T> {
    
    for ele in vec {
        if *ele == num {
            return Some(ele);
        }
    }
    return None;
}

fn find_element_in_two_vector<'a, T: std::cmp::PartialEq>(vec1: &'a Vec<T>, vec2: &'a Vec<T>, num: T) -> Option<&'a T> {
    
    for ele in vec1 {
        if *ele == num {
            return Some(ele);
        }
    }

    for ele in vec2 {
        if *ele == num {
            return Some(ele);
        }
    }

    return None;
}

fn main(){
    // let person = Person {
    //     name: String::from("jaikumar mohite"),
    //     age: 22
    // };

    // let json_str = serde_json::to_string(&person).unwrap();
    // println!("Serialized Json:: {}", json_str);

    // let deserialized_person: Person = serde_json::from_str(&json_str).unwrap();
    // println!("Deserialized Person: {:?}", deserialized_person);
    
    // let vec = vec![1, 2, 3, 4];
    // let vec1 = vec![3,4,5];
    // let num = find_element(&vec, 1);
    // println!("{:?}", num);

    // let num1 = find_element_in_two_vector(&vec, &vec1, 3);

    // println!("{:?}", num1);

    play_with_borsh();
}

// fn main() {
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

    // say_hello!("jaikumar");
    // hello();
// }

// fn print_name(user1: User) {
//     print!("User 1 username: {}", user1.active);
// }
// use std::fmt;
// #[derive(Debug, Clone)]
// struct User {
//     phone_number: u32,
//     age: u32,
// }

// impl fmt::Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} {}", self.name, self.age)
//     }
// }

// fn main() {
//     let user = User {
//         phone_number: 987654,
//         age: 20,
//     };
//     let new_user = user.clone();

//     println!("{:?}", user);
// }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User1{
    name: String,
    password: String
}


fn play_with_borsh(){
    let mut v: Vec<u8> = Vec::new();
    let user = User1{
        name: String::from("jaikumar"),
        password:String::from("123")
    };
    
    // serialized
    user.serialize(&mut v).unwrap();
    
    // deserialized
    let deserialised = User1::try_from_slice(&v).unwrap();

    print!("Serialized {:?}", v);
    
    print!("Deserialized {:?}", deserialised);

}


fn longest_string<'a, 'b>(str1: &'a String, str2: &'a String, str3: &'b String) -> &'a String {
    if(str1>str2){
        let temp = str3;
        print!("{}", temp);
        return str1;
    }
    str2
}

fn play_with_lifetime(){
    let str1 = String::from("jaikumar");
    let str2 = String::from("mohite"); 
    let ans: &String;
    {
        let str3 = String::from("paji");
        ans = longest_string(&str1, &str2, &str3);
    }

    print!("{}", ans);
}