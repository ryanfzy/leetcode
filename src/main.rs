mod solutions;

fn main() {
    use solutions::one;

    let nums = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let output = one::two_sum(nums, target);
    println!("{}, {}", output[0], output[1]);
}
