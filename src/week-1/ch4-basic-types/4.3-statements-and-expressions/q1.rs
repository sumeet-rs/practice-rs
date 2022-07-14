// One way
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x // return value
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }

// Another way 
 fn main() {
    let v = {
        let mut x = 1;
        x + 2 // return value
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }
 