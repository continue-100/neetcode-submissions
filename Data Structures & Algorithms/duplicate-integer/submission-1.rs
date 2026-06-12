impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        nums.iter().collect::<HashSet<_>>().len() != nums.len()
    }
}
