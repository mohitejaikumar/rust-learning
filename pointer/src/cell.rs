/*
2 points on Cell

1. No one has ref to what inside the cell 
2. There is single thread program so multiple places to mut but one mut at a time 

Allow for interior mutability 

Raw pointers = “You’re on your own. Rust will not protect you.”

suppose if get would have given ref outside 

--------------------------------------
let x = Cell::new(vec![1])

let y = &x.get()[0]

// change vector 
x.set(vec![1,2,3])

println!("{}", y)
--------------------------------------

This above code would have broke 
*/


/*
The only way in RUST to go from shared ref to exclusive ref is through UnsafeCell

&T -> &mut T , UnsafeCell


*/

use std::{cell::UnsafeCell, sync::Arc};


// this is impl unsafe to share the ref accross threads but was !Sync due to UnsafeCell
// will lead to interliving values 
unsafe impl<T> Sync for Cell<T> {}

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value:T) -> Self {
        Cell { 
            value: UnsafeCell::new(value)
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is curretly mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any reference, because we never give any out 
        unsafe {*self.value.get() = value;}
    }

    pub fn get(&self) -> T 
    where 
        T: Copy
    {   
        // SAFETY: we know that no one else is modifying this value, since only this thread can mutate
        // because !Sync and it is executing this function instead .
        // Rawpointer is unsafe because it can point to anything and bypass borrow checker and dangling pointer 
        unsafe { *self.value.get()}
    }
}


#[test]
fn bad1(){
    let x = Arc::new(Cell::new(42));

    let x1 = Arc::clone(&x);

    let y = Cell::new(5);
    let z = &y;
    z.set(7);

    println!("{}", y.get());

    std::thread::spawn(move || {
        x1.set(43);
    });

    let x2 = Arc::clone(&x);

    std::thread::spawn(move || {
        x2.set(44);
    });

}



