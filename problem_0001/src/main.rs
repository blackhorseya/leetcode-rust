use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let want = target - num;
            match map.get(&want) {
                None => {
                    map.insert(num, idx);
                }
                Some(&got_idx) => {
                    return vec![got_idx as i32, idx as i32];
                }
            }
        }

        unreachable!()
    }
}
