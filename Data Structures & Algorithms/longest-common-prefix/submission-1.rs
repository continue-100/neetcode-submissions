impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut arr: Vec<_> = strs.iter().map(|s| s.chars()).collect();
        let mut r = String::new();
        while let Some(chars) = arr.iter_mut().map(|v| v.next()).collect::<Option<Vec<_>>>() {
            if chars.iter().all(|c| *c == chars[0]) {
                r.push(chars[0]);
            } else {
                return r;
            }
        }
        r
    }
}