
// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if t { // Change !t to t to make this work
        println!("Success!");
    }
} 
