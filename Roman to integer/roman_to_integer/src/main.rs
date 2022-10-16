struct Solution {}

// 3ms, 2.3MB
// impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {
//         return s
//             .replace("IV", "IIII")
//             .replace("IX", "VIIII")
//             .replace("XL", "XXXX")
//             .replace("XC", "LXXXX")
//             .replace("CD", "CCCC")
//             .replace("CM", "DCCCC")
//             .chars()
//             .map(|c| match c {
//                 'I' => 1,
//                 'V' => 5,
//                 'X' => 10,
//                 'L' => 50,
//                 'C' => 100,
//                 'D' => 500,
//                 'M' => 1000,
//                 _ => 0
//             })
//             .sum()
//     }
// }

// 0ms, 2.1MB
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let mut roman_hash: HashMap<char, i32> = HashMap::new();

        roman_hash.insert('I', 1);
        roman_hash.insert('V', 5);
        roman_hash.insert('X', 10);
        roman_hash.insert('L', 50);
        roman_hash.insert('C', 100);
        roman_hash.insert('D', 500);
        roman_hash.insert('M', 1000);

        let s = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");


        return s.chars()
            .map(|c| roman_hash.get(&c).unwrap())
            .sum()

    }
}


fn main() {
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
}
