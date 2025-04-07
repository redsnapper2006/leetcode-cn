struct Solution {}

impl Solution {
  pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % 2 == 1 {
      return false;
    }
    let mut cache: Vec<Vec<i32>> = vec![];

    let mut idx: usize = 0;
    while idx < nums.len() {
      if nums[idx] > sum / 2 {
        return false;
      }
      if idx > 0 {
        cache.push(cache[idx - 1].clone());
        (0..=sum as usize / 2).for_each(|ii| {
          if ii as i32 + nums[idx] > sum / 2 || cache[idx - 1][ii] == 0 {
            return;
          }
          cache[idx][ii + nums[idx] as usize] = 1;
        });
      } else {
        cache.push(vec![0; sum as usize / 2 + 1]);
        cache[idx][nums[idx] as usize] = 1;
        cache[idx][0] = 1;
      }

      if cache[idx][sum as usize / 2] == 1 {
        return true;
      }
      idx += 1;
    }

    false
  }
}

fn main() {
  println!("{}", Solution::can_partition(vec![1, 5, 11, 5]));
  println!("{}", Solution::can_partition(vec![1, 2, 3, 5]));

  println!("{}", Solution::can_partition(vec![23, 13, 11, 7, 6, 5, 5]));
}
