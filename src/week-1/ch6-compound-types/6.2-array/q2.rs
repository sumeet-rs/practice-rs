
fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let _arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c']; // Array [Type=char; Size=3]
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12); // 4 * 3 = 12

    println!("Success!");
}
