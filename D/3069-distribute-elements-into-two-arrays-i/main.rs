impl Solution {
  pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut a1 : Vec<i32> = vec![nums[0]];
    let mut a2 : Vec<i32> = vec![nums[1]];

    (2..nums.len()).for_each(|idx|{
      if a1[a1.len()-1] > a2[a2.len()-1] {
        a1.push(nums[idx]);
      } else {
        a2.push(nums[idx]);
      }
    });


    a2.iter().for_each(|&v| {
      a1.push(v);
    });
    a1
  }
}
