
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // String can be concatenated with &str
    assert_eq!(s3,"hello,world!");
    println!("{}",s3); // s1 was already borrowed to s3. so replace s1 by s3
}
