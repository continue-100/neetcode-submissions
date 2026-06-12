impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let (mut a, mut b): (Vec<_>, Vec<_>) = (s.chars().collect(), t.chars().collect());
        a.sort();
        b.sort();
        for (i, j) in std::iter::zip(a, b) {
            if i != j {
                return false;
            }
        }
        true
    }
}
