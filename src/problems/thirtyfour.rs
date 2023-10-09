pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if !nums.contains(&target) {
        return vec![ -1, -1];
    }

    let i = nums.partition_point(|&x| x < target);
    let j = nums.partition_point(|&x| x <= target);
    if (nums.len() == 0) || ((nums[i] != target) && (nums[j-1] != target)) {
        return vec![ -1, -1];
    }
    return vec![i as i32, (j-1) as i32];
}