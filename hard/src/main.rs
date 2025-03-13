/*
------------------------------------------------MEMORY MANAGEMENT------------------------------------------------

2 memory
1. Stack
2. Heap

Rust program -> compiles -> binaries -> run this program

i32 willl take only 32 bits static memory at runtime stored in STACK

but String it is dynamic

HEAP                                          |                 STACK
+---------------------------------------------+-----------------------------------------+
| dynamic allocated at runtime                |     static allocated at compile time    |
|  much larger in size                        |    smaller in size                      |
|  slower                                     |    faster                               |
|  dynamic datastructure
(strings, vectores, hashmaps large stack  )   |    small variables of fixed size        |
+---------------------------------------------+-----------------------------------------+


In case of Strings reference of first char is stored in STACK (32 bit ) and actual content is stored in HEAP

SIZE of STACK FRAME needs to be fixed


What is memory maangement ?
allocation and deallocation memory .

GARBAGE COLLECTOR ->
js has garbage collector, cannot manually memory management

dangling pointers and all will not happen in rust

MANUAL ->
1. You allocate and deallocate memory yourself
2. can lead to dangling pointers
3. example : C

RUST WAY ->
1. It has its own ownership model for memory management
2. Makes it extremely safe to memory errors


------------------------------------MUTABILITY-------------------------------------------------------
By default, all variables are immutable in Rust.

let mut name = String::from("Jaikumar");
name.push_str(" Mohite");

-----------------------------------------OWNERSHIP-------------------------------------------------------------

Rust will force developer to write code in certain way to avoid memory issues

1. Ownership rules
    a. Each value in Rust has a variable that’s called its owner.
    b. There can only be one owner at a time.
    c. When the owner goes out of scope, the value will be dropped.

    example : what is issue of dangling pointers is multiple entity is pointing to same memory location


2. Moving
    





*/

// cloning saved us but its example of 
fn move_example() {
    // Error code 
    let s = String::from("Hello");
    let s2 = s.clone();
    println!("{}", s);
}


// ----------------------------------------------ERROR-------------------------------------------------------------

// fn do_something(s2: String) {
//     println!("{}", s2);

// }


// fn main() {

//     let s1 = String::from("Hello");
//     do_something(s1)
//     println!("{}", s1);
// }

// ----------------------------------------------MOVING-------------------------------------------------------------

// fn do_something(s2: String)-> String {
//     println!("{}", s2);
//     return s2;
// }


// fn main() {

//     let mut s1 = String::from("Hello");
//     s1 = do_something(s1);
//     println!("{}", s1);
// }


// ----------------------------------------------BORROWING / REFERENCING-------------------------------------------------------------

// Above code is ugly 

// fn main() {
//     let s1 = String::from("Hello"); // s1 is owner
//     let s2 = &s1;  // s1 is owner, s2 is borrowing
//     println!("{}", s1);    // s1 is owner
// }



// I want to borrow and mutate as well 

fn main() {
    let mut s1 = String::from("Hello"); // s1 is owner
    let s2 = &mut s1;  // s1 is owner, s2 is borrowing
    println!("{}", s1);    // s1 is owner
}

// Complex ----------> 

/*
1. if there is one mutable reference, there cannot be any mutable or immutable references
2. there can be any number of immutable references, if there is no mutable reference

*/