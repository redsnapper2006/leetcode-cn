use std::convert::TryInto;

struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sums = vec![0; nums.len()];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            sums[i] = sum;
        }
        let mut ret: i32 = -1;
        for i in 0..nums.len() {
            let mut p = 0;
            let n = sum - sums[i];
            if i > 0 {
                p = sums[i] - nums[i];
            }
            if p == n {
                ret = i.try_into().unwrap();
                break;
            }
        }
        ret
    }
}

fn main() {
    println!("helloworld");
    println!("{}", Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
}
