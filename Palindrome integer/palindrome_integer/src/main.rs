struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        let digit_count = 1 + (x as f32).log10() as u32;
        let half = (digit_count / 2) as usize;
        let digits = (0..digit_count).map(|exp| x / 10_i32.pow(exp) % 10);
        digits
            .clone()
            .take(half)
            .zip(digits.rev().take(half))
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(13212231));
}
