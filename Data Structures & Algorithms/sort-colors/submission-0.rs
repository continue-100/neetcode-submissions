impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count = [(0, 0), (1, 0), (2, 0)];
        for i in nums.iter() {
            count[*i as usize].1 += 1;
        }
        let sorted: Vec<i32> = count
            .into_iter()
            .flat_map(|(value, counter)| std::iter::repeat_n(value, counter))
            .collect();
        nums.clear();
        nums.extend(sorted);
    }
}