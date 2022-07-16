fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2]; // same as 0..2 Not specifying the first arg. will start from first index

    assert_eq!(slice1, slice2);

    println!("Success!");
}
