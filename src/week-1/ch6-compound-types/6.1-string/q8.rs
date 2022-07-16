// Approach 1
fn main() {
    let s =  "hello, world".to_string();
    let s1: &str = &s; // Borrow s to s1

    println!("Success!");
}

// Approach 2
fn main() {
    let s =  "hello, world"; // Remove .to_string() to make it a &str
    let s1: &str = s;

    println!("Success!");
}
