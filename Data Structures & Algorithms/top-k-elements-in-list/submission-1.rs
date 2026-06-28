impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for i in nums {
            if let Some(v) = count.get_mut(&i) {
                *v += 1;
            } else {
                count.insert(i, 1);
            };
        }
        let mut v = count.iter().collect::<Vec<_>>();
        v.sort_by_key(|(_k, v)| **v);
        v.reverse();
        v.truncate(k as usize);
        v.iter().map(|(k, _v)| **k).collect()
    }
}