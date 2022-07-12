fn main() {
    let x: i32 = 10;
    let y: i32 = 20; // Declare another variable y outside scope of inner block
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}
