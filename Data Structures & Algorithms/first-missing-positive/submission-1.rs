impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let set: HashSet<_> = nums.iter().filter(|&&i| i > 0).collect();
        let max = *nums.iter().max().expect("empty nums");
        if max < 1 {
            return 1;
        } else if max == 1 {
            return 2;
        }
        for i in 1..max {
            if !set.contains(&i) {
                return i;
            }
        }
        max + 1
    }
}