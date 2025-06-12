mod cell;
mod ref_cell;
mod rc;


/*

Smart pointers in Rust are objects that manage ownership and memory in a safe and 
automatic way, often providing extra features like reference counting, 
interior mutability, or thread-safety.


*const T , *mut T both are raw pointer , but difference is that 

*const T -> &T ✅
*const T -> &mut T ❌
*mut T -> &mut T ✅ 





*/