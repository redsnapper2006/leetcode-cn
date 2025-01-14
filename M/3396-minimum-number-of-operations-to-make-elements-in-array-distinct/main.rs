struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
      let mut cache : Vec<i32> = vec![0;101];
      let mut nums = nums;
      nums.iter().for_each(|&v|{
        cache[v as usize] += 1;
      });

      let mut ans : i32 = 0;
      while  cache.iter().filter(|x| **x > 1).count() > 0 {
        if nums.len() > 0 {
          cache[nums[0] as usize] -= 1;
        }
        if nums.len() > 1 {
          cache[nums[1] as usize] -= 1;
        }
        if nums.len() > 2 {
          cache[nums[2] as usize] -= 1;
        }
        ans += 1;
        nums = nums[3..].to_vec();
      }
      ans
    }
}

fn main() {
  println!("{}", Solution::minimum_operations(vec![1,2,3]));
}
