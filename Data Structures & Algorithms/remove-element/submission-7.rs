impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut back = nums.len() - 1;
        'arr: for i in 0..nums.len() {
            while nums[back] == val {
                if i >= back {
                    break 'arr;
                }
                back -= 1;
            }
            if nums[i] == val {
                nums.swap(back, i);
                back -= 1;
            }
            if i >= back {
                break;
            }
        }
        if nums[0] == val { 0 } else { back as i32 + 1 }
    }
}
