fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     let (s1, s2) = t.clone();
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
 