fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '6' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will passed: Each of the two UTF-8 chars '中' and '国'  occupies 3 bytes, 2 * 3 = 6
    // assert!(std::mem::size_of_val(&slice) == 6);
    assert!(std::mem::size_of_val(&slice) == 16);
        // Explanation: Pointer types to DSTs are sized but have twice the size of pointers to sized types.
        // So, the size of a slice is twice the size of the underlying array. 
        // Size of char pointer in 64 bit machines is 8 bytes, so 16 bytes is the size of slice. 
        // Ref: https://doc.rust-lang.org/reference/dynamically-sized-types.html

    println!("Success!");
}
