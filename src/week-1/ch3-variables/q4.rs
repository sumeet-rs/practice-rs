fn main() {
    println!("{}, world", define_x()); // Call define_x fn to get string 
}

fn define_x() -> String {
    let x = "hello";
    x.to_string() // Return string
}
