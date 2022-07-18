fn main() {
    for n in 1..100 { // remove = from 1..=100 to set boundary less than 100
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 
