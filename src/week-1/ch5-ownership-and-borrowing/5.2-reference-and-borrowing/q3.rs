
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); // Pass reference of s

    println!("Success!");
}

fn borrow_object(s: &String) {}
