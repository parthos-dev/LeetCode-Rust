pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solved = false;
    let mut out = vec![0 ,0];
    for i in 0..nums.len()-1 {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                out[0] = i as i32;
                out[1] = j as i32;
                break;
            }
        }
        if (solved){
            break;
        }
    }

    return out;
}