mod problems;
use problems::one;

fn main() {
    let nums = vec![2,7,11,15];
    let out = one::two_sum(nums, 9);
    println!("{:?}", out)
}
