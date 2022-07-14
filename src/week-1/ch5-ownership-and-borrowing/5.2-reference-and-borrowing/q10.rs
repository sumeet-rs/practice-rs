fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
//    let r2 = &mut s; // cannot borrow s as mutable more than once at a time
//    r2.push_str("!");
    
    println!("{}",r1);
}
