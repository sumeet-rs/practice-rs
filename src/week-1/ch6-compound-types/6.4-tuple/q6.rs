fn main() {
    let (x, y) = sum_multiply( (2, 3) );

    assert_eq!(x, 5); // 2 + 3 = 5
    assert_eq!(y, 6); // 2 * 3 = 6

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
