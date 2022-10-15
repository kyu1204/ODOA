struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut num_dict: HashMap<i32, i32> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            match num_dict.get(&(target - *num)) {
                Some(value) => return vec![*value, idx as i32],
                None => num_dict.insert(*num, idx as i32)
            };
        }
        vec![]
    }
}


fn main() {
    let result = Solution::two_sum(vec![3, 2, 4], 6);
    println!("{:?}", result);
}
