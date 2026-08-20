impl Solution {
  pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let (mut a1, mut a2) = (vec![nums[0]], vec![nums[1]]);
    for i in 2..nums.len() {
      let mut p = &mut a2;
      if *a1.last().unwrap() > *p.last().unwrap() {
        p = &mut a1;
      }
      p.push(nums[i]);
    }

    a1.extend(a2);
    a1
  }
}
