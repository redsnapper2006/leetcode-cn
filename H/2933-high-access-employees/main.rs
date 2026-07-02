impl Solution {
  pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
    let mut access_times: Vec<(String, i32)> = access_times
      .iter()
      .map(|x| (x[0].clone(), x[1].parse::<i32>().unwrap()))
      .collect();
    access_times.sort_unstable();

    let mut ans: Vec<String> = vec![];
    let mut valid: bool = false;
    for i in 2..access_times.len() {
      if access_times[i].0 != access_times[i - 2].0 {
        if valid {
          ans.push(access_times[i - 2].0.clone());
        }
        valid = false;
      } else if !valid && access_times[i].1 < access_times[i - 2].1 + 100 {
        valid = true;
      }
    }
    if valid {
      ans.push(access_times.last().unwrap().0.clone());
    }
    ans
  }
}
