impl Solution {
  pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
    let mut arr = arr;
    arr.sort_unstable();

    let mut work: Vec<i32> = vec![];
    for h2 in (0..4).rev() {
      if arr[h2] > 2 {
        continue;
      }
      work.push(arr[h2]);
      for h1 in (0..4).rev() {
        if h1 == h2 || work[0] == 2 && arr[h1] > 3 {
          continue;
        }
        work.push(arr[h1]);
        for m1 in (0..4).rev() {
          if m1 == h1 || m1 == h2 || arr[m1] > 5 {
            continue;
          }
          work.push(arr[m1]);
          for m2 in (0..4).rev() {
            if m2 == h1 || m2 == h2 || m2 == m1 {
              continue;
            }
            return format!("{}{}:{}{}", work[0], work[1], work[2], arr[m2]);
          }
          work.pop();
        }
        work.pop();
      }
      work.pop();
    }

    "".to_string()
  }
}
