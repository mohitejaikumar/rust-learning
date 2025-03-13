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
/*
Moving means transferring ownership from one variable to another.

Example:
let s1 = String::from("hello");
let s2 = s1; // s1's ownership is moved to s2
println!("{}", s1); // Error! s1 is no longer valid

This prevents multiple variables from pointing to the same memory location.
When s2 goes out of scope, it will free the memory since it owns the value.

To keep using s1, we need to clone the value instead of moving ownership:
let s2 = s1.clone();
*/





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

// AT this point what is difference between borrower and owner ?
// The key differences between borrower and owner are:

// 1. Owner has full control over the data and is responsible for memory management
//    - Can modify the data
//    - Handles cleanup when going out of scope
//    - Can only have one owner at a time

// 2. Borrower just has temporary access to read the data
//    - Cannot modify the data (unless mutable reference)
//    - Multiple immutable borrows allowed
//    - Does not affect memory management
//    - Must not outlive the owner

// Example:
// let s1 = String::from("Hello");    // s1 is owner
// let s2 = &s1;                      // s2 borrows from s1
// let s3 = &s1;                      // s3 also borrows from s1
// println!("{} {} {}", s1, s2, s3);  // All valid since s1 still owns the data




// I want to borrow and mutate as well 

fn main() {
    let mut s1 = String::from("Hello"); // s1 is owner
    let s2 = &mut s1;  // s1 is owner, s2 is borrowing
    s1.push_str(" World");
    println!("{}", s1);    // s1 is owner
}

// Complex ----------> 

/*
1. if there is one mutable reference, there cannot be any mutable or immutable references
2. there can be any number of immutable references, if there is no mutable reference

why 2nd rule, since there are two mutable person here one borrower and other is owner ?

    Actually, in the example above, there's a compilation error. The code won't run because Rust prevents this exact
    scenario. Here's why:

    1. When we create a mutable reference &mut s1 and assign it to s2, we temporarily give up the ability to use s1
    directly
    2. Then when we try to use s1.push_str(), the compiler stops us because we can't have both s1 and s2 
    modifying the same data
    3. This is by design in Rust and happens at compile time, before the OS gets involved

    While operating systems can handle concurrent access through locks and synchronization, 
    Rust takes a more strict approach by preventing data races at compile time. This:
    - Eliminates an entire class of bugs before the code even runs
    - Removes the runtime overhead of OS-level synchronization 
    - Makes concurrent code safer and easier to reason about

    The correct version would be:

    let mut s1 = String::from("Hello");
    let s2 = &mut s1;  
    s2.push_str(" World"); // Use s2 to modify
    println!("{}", s1);    // Now we can use s1 again after s2's scope ends


*/