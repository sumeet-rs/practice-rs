// Solution 1
fn main() {
    let s =  "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}",s)
}

// Solution 2
fn main() {
    let s =  "hello, world";
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
