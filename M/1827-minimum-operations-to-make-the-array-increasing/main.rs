struct Solution {}


impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
      let mut copy = nums.clone();
      let mut ret :i32 = 0;
      for i in 1..copy.len() {
        if copy[i] > copy[i-1] {
          continue;
        }
        ret +=  copy[i-1]+1-copy[i];
        copy[i] = copy[i-1]+1;
      }
      ret
    }
}