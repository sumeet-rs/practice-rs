struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Color) {
    let x:Color = p;
    assert_eq!(x.0, 0);
    assert_eq!(x.1, 127);
    assert_eq!(x.2, 255);
 }