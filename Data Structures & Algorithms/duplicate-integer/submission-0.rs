impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        let mut iter = nums.windows(2);
        while let Some([a, b]) = iter.next() {
            if a == b {
                return true;
            }
        }
        false
    }
}
