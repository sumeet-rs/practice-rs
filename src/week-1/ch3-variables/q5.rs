
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Change 5 to 12 to pass check and move forward
    }

    assert_eq!(x, 5); // Change 12 to 5 to pass check and move forward

    let x =  42;
    println!("{}", x); // Prints "42".
}
