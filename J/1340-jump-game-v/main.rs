impl Solution {
  pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    let mut stack: Vec<i32> = vec![];

    let mut left: Vec<i32> = vec![-1; arr.len()];
    let mut right: Vec<i32> = vec![-1; arr.len()];
    for i in 0..arr.len() {
      while stack.len() > 0 && arr[stack[stack.len() - 1] as usize] <= arr[i] {
        stack.pop();
      }
      left[i] = if stack.len() > 0 && i as i32 - stack[stack.len() - 1] <= d {
        stack[stack.len() - 1] as i32
      } else {
        -1
      };
      stack.push(i as i32);
    }
    stack.clear();
    for i in (0..arr.len()).rev() {
      while stack.len() > 0 && arr[stack[stack.len() - 1] as usize] <= arr[i] {
        stack.pop();
      }
      right[i] = if stack.len() > 0 && stack[stack.len() - 1] - i as i32 <= d {
        stack[stack.len() - 1] as i32
      } else {
        -1
      };
      stack.push(i as i32);
    }

    let mut ans: Vec<i32> = vec![0; arr.len()];
    fn dfs(idx: i32, left: &Vec<i32>, right: &Vec<i32>, ans: &mut Vec<i32>) -> i32 {
      if idx < 0 {
        return 0;
      }
      if ans[idx as usize] != 0 {
        return ans[idx as usize];
      }
      let v = dfs(left[idx as usize], left, right, ans).max(dfs(right[idx as usize], left, right, ans)) + 1;
      ans[idx as usize] = v;
      v
    }
    for i in 0..arr.len() {
      dfs(i as i32, &left, &right, &mut ans);
    }
    *ans.iter().max().unwrap()
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2));
  println!("{}", Solution::max_jumps(vec![3, 3, 3, 3, 3], 3));
  println!("{}", Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1));
  println!("{}", Solution::max_jumps(vec![7, 1, 7, 1, 7, 1], 2));
  println!("{}", Solution::max_jumps(vec![66], 1));
  println!(
    "{}",
    Solution::max_jumps(
      vec![
        59, 8, 74, 27, 92, 36, 95, 78, 73, 54, 75, 37, 42, 15, 59, 84, 66, 25, 35, 61, 97, 16, 6, 52, 49, 18, 22, 70,
        5, 59, 92, 85
      ],
      20
    )
  );
  println!(
    "{}",
    Solution::max_jumps(
      vec![
        22, 29, 52, 97, 29, 75, 78, 2, 92, 70, 90, 12, 43, 17, 97, 18, 58, 100, 41, 32
      ],
      17
    )
  );
}
