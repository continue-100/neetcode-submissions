impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut a = Vec::with_capacity(nums.capacity() * 2);
        for _ in 0..2 {
            for i in &nums {
                a.push(*i);
            }
        }
        a
    }
}
