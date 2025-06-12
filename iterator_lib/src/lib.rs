/*

trait Iterator {
// associated type 
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}


trait Iterator<Item> {
    fn next(&mut self) -> Option<Item>;
}


first one over formar because when we want to constraint to use one type of item on the 
for each type on which iterator implements .

*/

/*
fn main() {

    let vs = vec![1,2,3];
    /*
        implicit call to into_iter()  ownership is moved
        for v in vs {
            // v is owned values
        }
    */

    for v in vs.iter() {
        // v is borrowed values
    }

    for v in &vs {
        // v is borrowed values
    }
}
*/


/*

IntoIterator -> "You can convert me into something that iterates."
Iterator -> "I'm the thing that does the iterating."

*/

/*

Flatten
flatten the nested iterator 


*/



pub struct Flatten<O> 
where 
O: Iterator,
O::Item : IntoIterator,

{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>
}


impl<O> Flatten<O> 
where 
    O: Iterator,
    O::Item : IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Self {
            outer: iter,
            front_iter: None,
            back_iter: None
        }
    }
}

pub trait IteratorExt: Iterator {
    fn outer_flat(self) -> Flatten<Self>
    where 
        Self: Sized,
        Self:Iterator,
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T 
where 
    T: Iterator
{
    fn outer_flat(self) -> Flatten<Self>
    where 
        Self: Sized,
        Self: Iterator,
        Self::Item:IntoIterator
    {
        flatten(self)
    }
}


impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item : IntoIterator, // item is a type that implements IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = &mut self.front_iter {
                if let Some(next_item) = inner_iter.next() {
                    return Some(next_item);
                }
                self.front_iter = None;
            }

            if let Some(next_item) = self.outer.next() {
                self.front_iter = Some(next_item.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}


// iterator 
// intoIterator -> creates IntoIter which is an Iterator itself


impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item : IntoIterator,
    <O::Item as IntoIterator>::IntoIter : DoubleEndedIterator,
{   
    
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(back_iter) = &mut self.back_iter {
                if let Some(back_item) = back_iter.next_back() {
                    return Some(back_item);
                }
                self.back_iter = None;
            }

            if let Some(next_back_item) = self.outer.next_back() {
                self.back_iter = Some(next_back_item.into_iter());
            } else {
                
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}


pub fn flatten<O>(item: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item : IntoIterator,
{   
    Flatten::new(item.into_iter())
}


#[test]
fn test_empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
}

#[test]
fn test_one() {
    assert_eq!(flatten(std::iter::once(vec![1])).count(), 1);
}

#[test]
fn test_two() {
    assert_eq!(flatten(vec![vec![1,2], vec![3,4]]).count(), 4);
}


#[test]
fn test_two_empty() {
    assert_eq!(flatten(vec![Vec::<()>::new(),vec![], vec![]]).count(), 0);
}

#[test]
fn test_reverse() {
    assert_eq!(
        flatten(std::iter::once(vec!["a", "b"]))
        .rev()
        .collect::<Vec<_>>(),
        vec!["b", "a"]
    );
}

#[test]
fn test_reverse_error() {
    let mut iter = flatten(vec![vec!["a", "b", "e"], vec!["c", "d"]]);
    assert_eq!(iter.next(), Some("a"));
    assert_eq!(iter.next_back(), Some("d"));
    assert_eq!(iter.next(), Some("b"));
    assert_eq!(iter.next_back(), Some("c"));
    assert_eq!(iter.next(), Some("e"));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn deep_flatten(){
    assert_eq!(flatten(flatten(vec![vec![vec![1,2]]])).count(), 2);
}

#[test]
fn test_extension(){
    assert_eq!(vec![vec![1,2], vec![3,4]].into_iter().outer_flat().count(), 4);
}