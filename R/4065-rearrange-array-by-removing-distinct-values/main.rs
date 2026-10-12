impl Solution {
  pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut buf: [i32; 101] = [0; 101];
    nums.iter().for_each(|&v| {
      buf[v as usize] += 1;
    });

    let mut ans: Vec<i32> = vec![];
    let mut hit = true;
    while hit {
      hit = false;
      for i in 0..101 {
        if buf[i] > 0 {
          ans.push(i as i32);
          buf[i] -= 1;
          hit = true;
        }
      }
    }
    ans
  }
}
