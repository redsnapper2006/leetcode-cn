impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut multi : Vec<i32> = vec![0;nums.len()];
    let mut add : Vec<i32> =vec![0;nums.len()];

    for i in 0..nums.len() {
      let mut v = nums[i];
      while v > 0 {
        if v % 2 == 1 {
          add[i] +=1 ;
          v -=1;
        }  else {
          v /=2;
          multi[i] +=1;
        }
      }
    }
    multi.iter().max().unwrap() + add.iter().sum::<i32>()
  }
}
