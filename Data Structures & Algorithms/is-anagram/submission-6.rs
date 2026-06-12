impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut bits = [0u8; 256];
        for (c1, c2) in std::iter::zip(s.chars(), t.chars()) {
            bits[c1 as usize] += 1;
            bits[c2 as usize] -= 1;
        }
        bits == [0u8; 256]
    }
}