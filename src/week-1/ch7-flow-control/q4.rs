// Fix the errors without adding or removing lines
fn main() {
    let names = ["liming","hanmeimei"]; // Change type from String to str. as String is dynamic heap memory, the assignmemnt moves from names to name. str is static memory. So, It's a copy.
    for name in names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
    }
    
    println!("{:?}", numbers);
} 
