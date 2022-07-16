fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3]; // Single char 3 bytes 

    assert!(slice == "你");

    println!("Success!");
}
