
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