struct Solution {}

impl Solution {
  pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    if arr.len() == 1 {
      return 1;
    }
    let mut ret: i32 = 1;
    let mut start: usize = 0;
    let mut isAsc: bool = false;
    let mut isSet: bool = false;
    for i in 1..arr.len() {
      if !isSet {
        if arr[i] != arr[i - 1] {
          isAsc = arr[i] > arr[i - 1];
          start = i - 1;
          isSet = true;
        }
      } else {
        if arr[i] == arr[i - 1] {
          isSet = false;
          if (i - start) as i32 > ret {
            ret = (i - start) as i32;
          }
        } else if isAsc == (arr[i] > arr[i - 1]) as bool {
          if (i - start) as i32 > ret {
            ret = (i - start) as i32;
          }
          start = i - 1;
        } else {
          isAsc = arr[i] > arr[i - 1];
        }
      }
    }
    if isSet && (arr.len() - start) as i32 > ret {
      ret = (arr.len() - start) as i32;
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::max_turbulence_size(vec![1]));
}
