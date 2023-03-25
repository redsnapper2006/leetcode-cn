struct Solution {}

impl Solution {
  pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut start: usize = 0;
    let mut end: usize = arr.len() - 1;
    while start < arr.len() {
      if start < arr.len() - 1 && arr[start] > arr[start + 1] {
        break;
      }
      start += 1;
    }
    if start == arr.len() {
      return 0;
    }
    while end >= 0 {
      if end == 0 || end > 0 && arr[end] < arr[end - 1] {
        break;
      }
      end -= 1;
    }
    if end == 0 {
      return 0;
    }

    let mut ret: usize = arr.len();
    let mut s = start;
    while s >= 0 {
      let v = arr[s];
      let mut rs: usize = end;
      let mut re: usize = arr.len() - 1;
      while rs <= re {
        let m = rs + (re - rs) / 2;
        if v > arr[m] {
          rs = m + 1;
        } else {
          re = m - 1;
        }
      }
      if ret > rs - s - 1 {
        ret = rs - s - 1;
      }
      if s == 0 {
        break;
      }
      s -= 1;
    }

    let mut e = end;
    while e < arr.len() {
      let v = arr[e];
      let mut ls: usize = 0;
      let mut le: usize = start;
      while ls <= le {
        let m = ls + (le - ls) / 2;
        if v >= arr[m] {
          ls = m + 1;
        } else {
          if m == 0 {
            break;
          }
          le = m - 1;
        }
      }
      if ret > e - ls {
        ret = e - ls;
      }

      e += 1;
    }

    ret as i32
  }
}
