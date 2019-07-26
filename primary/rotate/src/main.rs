//
//    Example 1:
//
//    Input: [1,2,3,4,5,6,7] and k = 3
//    Output: [5,6,7,1,2,3,4]
//    Explanation:
//    rotate 1 steps to the right: [7,1,2,3,4,5,6]
//    rotate 2 steps to the right: [6,7,1,2,3,4,5]
//    rotate 3 steps to the right: [5,6,7,1,2,3,4]
//    Example 2:
//
//    Input: [-1,-100,3,99] and k = 2
//    Output: [3,99,-1,-100]
//    Explanation:
//    rotate 1 steps to the right: [99,-1,-100,3]
//    rotate 2 steps to the right: [3,99,-1,-100]
//    Note:
//
//    Try to come up as many solutions as you can, there are at least 3 different ways to solve this problem.
//    Could you do it in-place with O(1) extra space?


struct Solution {}

fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    println!("num: {:?}", nums);
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for i in 1..=k {
            let remove = nums.remove(nums.len() - 1 as usize);
            nums.insert(0, remove);
//            println!("{}", i)
        }
    }
}
