/*

RefCell -> dynamically check borrowing in run-time , think ? of graph problem 

it itself figure out borrow checking at runtime .

this is not thread safe 

Phase -> 

when return type is Option<&T> and Option<&mut T> this never decrease 
like if at some point i remove exclusive shared ref then i cannot have shared ref 
anymore .


*/

use std::cell::UnsafeCell;
use crate::cell::Cell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}


// shared ref and want to mutate
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    reference: Cell<RefState>
}



impl<T> RefCell<T>{ 
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            reference: Cell::new(RefState::Unshared)
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_,T>> {
        
        match self.reference.get() {
            RefState::Unshared => {
                self.reference.set(RefState::Shared(1));
                Some(Ref{refcell:self})
            }

            RefState::Shared(i) => {
                self.reference.set(RefState::Shared(i+1));
                Some(Ref{refcell:self})
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.reference.get() {
            self.reference.set(RefState::Exclusive);
            // SAFETY: no other references have been given out since state would be 
            // Shared or Exclusive 
            Some(RefMut{refcell:self})
        }
        else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    // invoked when calling function on the ref of the type, help to behave it like a
    // reference .
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // a Ref is only created if no exclusive references have been given out
        // once it is given out, state is set to Shared, so no exclusive references are
        // given out .
        // dereferencing into a shared reference is fine .
        unsafe { &*self.refcell.value.get()}
    }
}

impl<T> Drop for Ref<'_, T>{
    fn drop(&mut self) {
        match self.refcell.reference.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.reference.set(RefState::Unshared);
            },
            RefState::Shared(n) => {
                self.refcell.reference.set(RefState::Shared(n-1));
            }
        }
    }
}


pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    // invoked when calling function on the ref of the type
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get()}
    }
}


impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        // SAFETY
        // a RefMut is only created if no other references have been given out
        // once it is given out, state is set to Exclusive, so no futute references are
        // given out .
        // so we have an exclusive lease on the inner value, so mutably dereferencing 
        // is fine .
        unsafe { &mut *self.refcell.value.get()}
    }
}

impl<T> Drop for RefMut<'_, T>{
    // triggered when variable moves out of scope 
    fn drop(&mut self) {
        match self.refcell.reference.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => {
                self.refcell.reference.set(RefState::Unshared);
            },
        }
    }
}