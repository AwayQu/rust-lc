//
//    Say you have an array for which the ith element is the price of a given stock on day i.
//
//    Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
//
//    Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
//
//    Example 1:
//
//    Input: [7,1,5,3,6,4]
//    Output: 7
//    Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
//    Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
//    Example 2:
//
//    Input: [1,2,3,4,5]
//    Output: 4
//    Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//    Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//    engaging multiple transactions at the same time. You must sell before buying again.
//    Example 3:
//
//    Input: [7,6,4,3,1]
//    Output: 0
//    Explanation: In this case, no transaction is done, i.e. max profit = 0.
//

struct Solution {

}

fn main() {
    println!("max_profit: {:?}", Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("max_profit: {:?}", Solution::max_profit(vec![1,2,3,4,5]));
    println!("max_profit: {:?}", Solution::max_profit(vec![7,6,4,3,1]));
}


impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut b_index:usize = 0;
        let mut seek_index:usize = 0;

        let mut max_profit= 0;
        let mut buyed: bool = false;
        for x in prices.to_owned() {
            seek_index += 1;
            match prices.get(seek_index).to_owned() {
                Some(maySel) => {
                    let may_sel_val = *maySel;
                    if may_sel_val > x {
                        if !buyed {
                            buyed = true;
                            b_index = seek_index - 1;
                        }
                    } else if may_sel_val < x {
                        if buyed {
                            buyed = false;
                            max_profit += x - &prices[b_index]
                        }
                    }
                },
                None => {
                    if buyed {
                        buyed = false;
                        max_profit += x - &prices[b_index]
                    }
                }
            }
        }

        return max_profit
    }
}