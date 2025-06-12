/*

rc -> it help to have multiple shared reference to a thing and only deallocate
when last one goes out .
think of as big blob used at multiple places in my single threaded program


this is thread unsafe .
we want it to be saved on heap .

*/

use std::{marker::PhantomData, ptr::NonNull};

use crate::cell::Cell;

struct RcInner<T>{
    value: T,
    refcount: Cell<usize> 
}

struct Rc<T>{
    // NonNull gives us safety that raw pointer do not point to null 
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>
}


impl<T> Rc<T> {

    pub fn new(value:T) -> Self {
        // on Heap
        let inner = Box::new(RcInner{
            value,
            refcount: Cell::new(1)
        });

        Rc {
            // SAFETY: box doesnot give us null pointer
            inner: unsafe {NonNull::new_unchecked(Box::into_raw(inner))},
            _marker: PhantomData
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY: inner is Box which will be only deallocated when last Rc goes away 
        // we have an Rc that means Box is not deallocated so its fine to dereference 
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe {self.inner.as_ref()};
        let new_count = inner.refcount.get();
        inner.refcount.set(new_count + 1);

        Rc {
            inner: self.inner,
            _marker: PhantomData
        }
    }
}



impl<T> Drop for Rc<T>{

    fn drop(&mut self) {
        let inner = unsafe{ self.inner.as_ref()};
        let counter = inner.refcount.get();

        if counter == 1 {
            let _ = unsafe {Box::from_raw(self.inner.as_ptr())};
        } else {
            inner.refcount.set(counter - 1);
        }
    }
}
