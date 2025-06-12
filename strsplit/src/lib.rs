/*
Progress with time -> 

single lifetime -> double lifetime -> generics with traits and trait bounds 

*/

pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

// str similar to seq of char dont know the len [char] 
// str -> it is unsized UTF-8 encoded text in memory always used as &str
// &str is immutable , borrowed view in (String, literal, other string view) , has start pointer and len, it points to heap and stack both 
// &str .len() -> return bytes not char count 
// String is heap allocated, know start pointer and len 
// String -> &str because know the len (cheap -- AsRef)
// &str -> String (we have to do heap allocation and copy the string)(expensive--memcpy)






// this impl block is generic over the lifetime 'a
impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}


pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}


impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}


impl Delimiter for &String {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(*self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
        .find(|(_, c)| c == self)
        .map(|(start, _)| (start, start + 1))
    }
}


impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where D: Delimiter
{
    type Item = &'haystack str;

    // THIS IS V1

    // fn next(&mut self) -> Option<Self::Item> {
    //     if let Some(next_delim) = self.remainder.find(self.delimiter) {
    //         let until_delimiter = &self.remainder[..next_delim];
    //         self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
    //         Some(until_delimiter)
    //     } else if self.remainder.is_empty() {
    //         // TODO: bug
    //         None
    //     } else {
    //         let rest = self.remainder;
    //         self.remainder = "";
    //         // &'a str     &'static str 
    //         // you can extend lifetime of some variable, but you cannot shorten it
    //         Some(rest)
    //     }
    // }

    // THIS IS V2 
    /*
    where:
        'delimeter: 'haystack ('delimiter is greater than haystack)
    

    */
    fn next(&mut self) -> Option<Self::Item> {

        if let Some(ref mut remainder) = self.remainder {
            if let Some((next_start, next_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..next_start];
                *remainder = &remainder[next_end..];
                Some(until_delimiter)
            } else{
                
                /*
                this function is implemented on Option<T> if None then return None
                and if Some then return Some(T) but set None. 
                 */
                self.remainder.take()
            }
        } else {
            None
        }
    }
} 

// lifetime for value exists until it goes out of scope and it is not moved 



// if 2 arg passed of diff liftime but wants both arg to have same lifetime then it 
// take lifetime of shorter arg 

// anonymous lifetime to infer itself
pub fn until_char(s: &str, c: char) -> & '_ str {
    StrSplit::new(s, c)
    .next()
    .expect("StrSplit never returns None")
}


#[test]
fn until_char_works() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}


#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
} 

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters,vec!["a", "b", "c", "d", ""]);
} 