struct Solution {}

impl Solution {
  pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
    let mut idxs: Vec<usize> = vec![arr.len() - 1];

    let mut idx: usize = arr.len() - 2;
    let mut change: usize = 0;
    loop {
      if arr[idx] <= arr[idx + 1] {
        if arr[idx] == arr[idx + 1] {
          let s = idxs.len() - 1;
          idxs[s] = idx;
        } else {
          idxs.push(idx);
        }
        if idx == 0 {
          break;
        }
        idx -= 1;
        continue;
      }

      let mut stack_idx: usize = idxs.len() - 1;
      while arr[idx] > arr[idxs[stack_idx]] {
        if stack_idx == 0 {
          break;
        }
        stack_idx -= 1;
      }

      if stack_idx == 0 && arr[idx] > arr[idxs[0]] {
        change = idxs[0];
      } else {
        change = idxs[stack_idx + 1];
      }
      break;
    }
    let mut ret = arr;
    let t = ret[change];
    ret[change] = ret[idx];
    ret[idx] = t;
    ret
  }
}

fn main() {
  // println!("{:?}", Solution::prev_perm_opt1(vec![3, 2, 1]));
  // println!("{:?}", Solution::prev_perm_opt1(vec![1, 1, 5]));
  println!("{:?}", Solution::prev_perm_opt1(vec![1, 9, 4, 6, 7]));
  println!("{:?}", Solution::prev_perm_opt1(vec![3, 1, 1, 3]));
}
