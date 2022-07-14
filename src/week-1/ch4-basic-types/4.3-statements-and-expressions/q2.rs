
fn main() {
    let v = { // Replace parentheses with curly braces
         let x = 3;
         x
    };
 
    assert!(v == 3);
 
    println!("Success!");
 }