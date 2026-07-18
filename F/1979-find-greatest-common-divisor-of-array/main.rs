impl Solution {
  pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let gcd = |mut x: i32, mut y: i32| -> i32 {
      while y != 0 {
        let t = y;
        y = x % y;
        x = t;
      }
      x
    };

    gcd(*nums.iter().min().unwrap(), *nums.iter().max().unwrap())
  }

  pub fn find_gcd2(nums: Vec<i32>) -> i32 {
    let mut mn = nums[0].min(nums[1]);
    let mut mx = nums[0].max(nums[1]);

    (2 - nums.len() % 2..nums.len()).step_by(2).for_each(|idx| {
      let (mut x, mut y) = (nums[idx], nums[idx + 1]);
      if x > y {
        (x, y) = (y, x);
      }
      if x < mn {
        mn = x;
      }
      if y > mx {
        mx = y;
      }
    });

    let gcd = |mut x: i32, mut y: i32| -> i32 {
      while y != 0 {
        let t = y;
        y = x % y;
        x = t;
      }
      x
    };

    gcd(mn, mx)
  }
}
