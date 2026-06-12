impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut set = HashMap::new();
        for i in strs {
            let mut s: Vec<_> = i.chars().collect();
            s.sort();
            set.entry(s)
                .and_modify(|v: &mut Vec<_>| v.push(i.clone()))
                .or_insert(vec![i]);
        }
        set.into_values().collect()
    }
}