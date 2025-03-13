use std::collections::HashMap;
// fn main() {
//     println!("Hello, world!");

//     let mut a = Vec::new();
//     a.push(1);
//     a.push(2);
//     a.push(3);
//     a.push(4);
//     a.push(5);
//     a.push(6);

//     let even_numbers = get_even_numbers(&a);
//     println!("{:?}", even_numbers);
//     println!("{:?}", a);

//     let vector_tuple = vec![
//         (String::from("harkiat"), 21),
//         (String::from("jaikumar"), 32),
//         (String::from("shubham"), 32),
//         (String::from("himanshu"), 22),
//         (String::from("jaikumar"), 324),
//     ];
//     let hashmap = get_hashmap(&vector_tuple);

//     println!("{:?}", hashmap);
// }

// struct Job<T, U>{
//     id: T,
//     name: U
// }

// impl<T, U> Job<T, U>{
//     fn new(id: T, name: U) -> Self{
//         Self{id, name}
//     }
// }
// static A: Job<i32, &'static str> = Job { id: 1, name: "jaikumar" };
// println!("{:?}", a);

fn get_even_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let mut even_numbers = Vec::<i32>::new();

    for number in numbers {
        if number % 2 == 0 {
            even_numbers.push(*number);
        }
    }
    even_numbers
}

fn get_hashmap(temp_vector: &Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut hashmap = HashMap::<String, Vec<i32>>::new();
    let mut temp_iter = temp_vector.iter();
    while let Some(val) = temp_iter.next() {
        let key_name = hashmap.get_mut(&val.0);
        match key_name {
            Some(val_vec) => val_vec.push(val.1),
            None => {
                let temp_vec = vec![val.1];
                hashmap.insert(val.0.clone(), temp_vec);
            }
        }
    }

    return hashmap;
}

// dynamic in size is called collections

/*

Collections

ex -> Vector

Vec::new()
v.push()
v.remove(//index)
vec![1,2,3]
println!("{:?", vec)
vec.get(//index)

// generics in vector
Vec::<i32>::new()


// Hashmap

insert
get_mut
get
or_insert_with

*/

// --------- ----------------------------------------------Iterator-------------------------------------------------------

/*

// Iterator in COLLECTIONS (vector, hashmap)

    DEFAULT : into_iter


    1. vec.iter()

    doesn't do aything until function is called
    it borrows the values, no ownership transfer

    2. vec.iter_mut()

    iteratable mutable , no ownership transfer

    // next

    let iter_temp = vec.iter();

    while let Some(val) = iter.next() {
        println!("{:?}", val)
    }

    3. vec.into_iter()

    transfer ownership , sure that collection will not be used further

// Consuming adaptors are methods which will consume iterator and returns value
// Iterator adaptors , they produce different iterator , also change some aspects of original iterator and consume origianl iterator



*/

fn iterators_info() {
    let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();
    // // Consuming adaptors / self is passed in this sum function , not &self
    // let sum: i32 = v1_iter.sum();

    // println!("{:?}", sum);

    // // v1_iter is consumed
    // println!("{}", v1_iter);

    let v2_iter = v1.iter();

    // Iterator adaptor
    let v4_iter = v2_iter.filter(|x| **x % 2 == 0);
    let v3_iter = v4_iter.map(|x| *x + 1);
}

fn iterator_assignment() {
    let temp_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let new_iter = temp_vec.iter().filter(|x| **x % 2 == 0).map(|x| *x * 2);

    // create new vector from iterator
    // explicitly give the type
    let new_vector: Vec<i32> = new_iter.collect();
}

//  ----------------------------------------------------STRING & SLICES -----------------------------------------------------------

// growable , mutable , owned, UTF-8 encoded string type

/*


****String****

1. ptr, len , capacity  (in Stack)


****Slice*****

Slice is reference a contiguous sequence in Collection
reference but not ownership
its generic to vector, hashmap , string

=> ptr, len
----------------------------|
ex ----> str = &word[0..3]  |
                            |
string_slice Type = &str    |
----------------------------|

It is `view` of the string
SOLVES : -
    1. no extra memory
    2. what if original words cahnged , then this also changes


******String Literals*******

let words = "hello world!";

=>  stored in binaries and `words` has &str in this binary


*/

fn string_assignment(str: &String) -> &str {
    let word2 = &str[0..5];
    return word2;
}

// ---------------------------------------------------------Traits------------------------

/*

****traits*******  is like blueprint to types

something like `interface` in javascript
we can share it between types

****trait bounds*****

for generic to have certain behaviour (syntaxtical sugar )

T: trait1 + trait2






*/

trait Summary {
    fn summarise(&self) -> String {
        return String::from("Summarize");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("The name is {}, and the age is {}", self.name, self.age);
    }
}

impl User {
    fn name_me(&self) -> String {
        return format!("{}", String::from("jaikumar"));
    }
}

fn main() {
    let user = User {
        name: String::from("Jaikumar Mohite"),
        age: 21,
    };

    println!("{:?}", user.name_me());
    notify2(user);
}

// argument should have type with trait implemented on that
fn notify(u: impl Summary) {
    // if want reference &impl Summary
    println!("{}", u.summarise());
}

trait Fix {
    fn summarize1(&self) {
        println!("{}", String::from("ibvubwibev"))
    }
}
impl Fix for User {}
fn notify2<T: Summary + Fix>(item: T) {
    println!("{}", item.summarise());
    println!("{:?}", item.summarize1());
}
