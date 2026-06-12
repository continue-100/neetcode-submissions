impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            if let Some(a) = nums[i + 1..].iter().position(|e| *e == target - nums[i]) {
                return vec![i as i32, (a + i + 1) as i32];
            }
        }
        unreachable!()
    }
}