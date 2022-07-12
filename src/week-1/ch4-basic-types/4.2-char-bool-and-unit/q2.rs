fn main() {
    let c1 = "中";
    print_char(&c1); // Pass c1 as reference
} 

fn print_char(c : &str) { 
    println!("{}", c);
}
