fn main() {
    let f = false; // Change true to false
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}