// Approach 1
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = String::clone(&x);
    println!("{},{}",x,y);
}

// Approach 2
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
