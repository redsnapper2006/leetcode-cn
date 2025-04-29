use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
  pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let  set : HashSet<i32> =  HashSet::from_iter(nums.iter().cloned());

    let mut sum : i32 = nums[0];
    let mut idx : usize = 1;
    while idx < nums.len() {
      if nums[idx]!=nums[idx-1]+1 {
        break;
      }

        sum += nums[idx];

      idx+=1;
    }
    while set.contains(&sum) {
      sum+=1;
    }
    sum
  }
}

struct Solution{}
fn main(){
  println!("{}", Solution::missing_integer(vec![1,2,3,14,13,12]));
}
