// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(&s); // Pass s as reference to print_str function

    println!("{}", s);
}

fn print_str(s: &String)  { // Set reference type as &String
    println!("{}",s)
}
