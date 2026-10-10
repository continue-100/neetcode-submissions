impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre = vec![0];
        let mut sum = 0;
        for i in &nums {
            sum += i;
            pre.push(sum)
        }

        let mut c = 0;

        for i in 0..pre.len() {
            for &n in pre[i + 1..].iter() {
                if n - pre[i] == k {
                    c += 1
                }
            }
        }
        c
    }
}