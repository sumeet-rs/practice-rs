// Approach 1: Using both Box type
fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(s)
}

fn greetings(s: Box<str>) {
    println!("{}",s)
}


// Approach 2: Using both &str type
fn main() {
    let s: &str =  "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
