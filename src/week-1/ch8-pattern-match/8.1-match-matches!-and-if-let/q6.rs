
fn main() {
    let o = Some(7);

    // Match Block
    // match o {
    //     Some(i) => {
    //         println!("This is a really long string and `{:?}`", i);

    //         println!("Success!");
    //     }
    //     _ => {}
    // };
    
    // Using if let block
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
};
}
