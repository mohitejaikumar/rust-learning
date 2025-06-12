macro_rules! jaikumar {
    ($elem1:ty=>$elem_value:expr => $elem2:ty=>$elem_value2:expr => $elem3:ty=>$elem_value3:expr) => {
        let a: $elem1 = $elem_value;
        let b: $elem2 = $elem_value2;
        let c: $elem3 = $elem_value3;
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
    };
}

fn main() {
    jaikumar!(i32=>10 => i32=>20 => i32=>30);
}
