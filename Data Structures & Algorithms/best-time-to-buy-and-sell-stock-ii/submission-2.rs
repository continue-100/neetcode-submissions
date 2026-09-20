impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .map(|w| match w {
                &[a, b] => (b - a).max(0),
                _ => 0,
            })
            .sum()
    }
}
