//Given a non-empty array of integers, every element appears twice except for one. Find that single one.
//
//Note:
//
//Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
//
//Example 1:
//
//Input: [2,2,1]
//Output: 1
//Example 2:
//
//Input: [4,1,2,1,2]
//Output: 4

struct Solution {}
fn main() {
    println!("v : {}", Solution::single_number(vec![1,1,2,2,3]));
}


impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res:i32 = 0;
        for x in nums {
            res = x ^ res;
        }
        res
    }
}