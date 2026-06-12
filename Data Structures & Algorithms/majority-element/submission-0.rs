impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in nums {
            map.entry(i).and_modify(|c| *c += 1).or_insert(1);
        }
        *map.iter().max_by_key(|(_k, v)| **v).expect("no majority").0
    }
}