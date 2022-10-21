struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("")
        }
        else if strs.len() == 1 {
            let result = strs[0].clone();
            return result
        }
        else {
            let mut pre = strs[0].clone();

            for i in strs {
                while ! i.starts_with(&pre) {
                    pre = String::from(&pre[..pre.len()-1])
                }
            }

            return pre;
        }
    }
}



fn main() {
    println!("{}", Solution::longest_common_prefix(vec![String::from("teacher"), String::from("tesla"), String::from("test")]))

}
