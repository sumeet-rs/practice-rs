fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y); // Access value at address y

    println!("Success!");
}
