use std::{ops::Deref, pin::Pin};


impl<P> Deref for Pin<P>
where
    P: Deref,
    P::Target: T
{
    type Target = P::Target;

    fn deref(&self) -> &Self::Target {
        &*self.ptr
    }
}

fn main() {
    println!("Hello, world!");
}
