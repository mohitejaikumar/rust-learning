/*
Progress with time -> 

single lifetime -> double lifetime -> generics with traits and trait bounds 

*/

pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

// str similar to seq of char dont know the len [char] 
// &str has start pointer and len, it points to heap and stack both 
// String is heap allocated, know start pointer and len 
// String -> &str because know the len (cheap -- AsRef)
// &str -> String (we have to do heap allocation and copy the string)(expensive--memcpy)






// this impl block is generic over the lifetime 'a
impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}



impl<'haystack> Iterator for StrSplit<'haystack, '_> {
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
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
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
    let delim = format!("{}", c);
    StrSplit::new(s, &delim)
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