fn main() {
    let mut s =  String::from("hello");
    s.push(',');
    s += &" world".to_string();
    s.push('!');

    println!("{}", s);
}
