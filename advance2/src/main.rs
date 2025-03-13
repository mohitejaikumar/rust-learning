use std::{
    sync::mpsc,
    thread::{self, spawn},
};

// fn main() {
//     println!("Hello, world!");
//     let str1 = String::from("beiorvboeriovn");

//     let ans;
//     // {
//         let str2 = String::from("Hello Jaikumar");
//         ans = longest(&str1, &str2);
//     // }

//     println!("{}", ans);
// }

// compiler dont know which one will be returned s1 or s2;

// fn longest(s1: &str, s2: &str) -> &str {
//     if s1.len() > s2.len() {
//         return s1;
//     }
//     return s2;
// }

// return time have shorter lifetime
// force compiler to have 'a -> to be lifetime where both s1 and s2 are valid and return type will also have same lifetime
// 'a is lifetime relationship between s1, s2 and return

fn longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}

// ------------------------------------------Lifetimes------------------------------------------------------------

/*
generic lifetime annotation -> <'a>
Why ? ------> get nice error message




something like this
fn longest(s1&str (line4 to line 12))


whenever there is reference of variables , there should be role of `lifetime` , compiler should know
relationship of that references


*/

// lifetime ke bug vala code kuch kaise mai kaam karta hai but sare case mai nahi karta to compiler ko
// relationship batani padti hai

// ---------------------------------------------------------------------------------------\
// nahi to kya hoga compiler us case mai bhi error deta hai jismai kaam karna chahiye tha \
// ---------------------------------------------------------------------------------------\

struct User<'a> {
    name: &'a str,
}

fn get_user() {
    let name = String::from("jaikumar Mohite");
    let user = User { name: "Jaikumar" };

    println!("{}", user.name);
}

// -----------------------------------------------------Multithreading-----------------------------------------

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 0..2 {
    //         println!("{}", i);
    //     }
    // });

    // for i in 2..4 {
    //     println!("{}", i);
    // }

    // // something like wait group in GO
    // handle.join().unwrap();

    // let x = 1;

    // let v = vec![1, 2, 3];
    // // transfer ownership or move
    // thread::spawn(move || {
    //     println!("{:?}", v);
    // });
    // println!("{}", x);
    findsum();
}

// channels help in communication between the threads , transmitter and receiver .
// mpsc -> multiple producer and single consumer .

fn findsum() {
    let (tx, rev) = mpsc::channel();
    let mut sum:u64 = 0;
    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut temp_sum = 0;
            for j in 0..10000000 {
                temp_sum += (i * 10000000 + j);
            }
            producer.send(temp_sum).unwrap();
        });
    }
    drop(tx);
    for val in rev {
        sum+=val;
    }
    println!("{}", sum);
}

// macro write code for you 