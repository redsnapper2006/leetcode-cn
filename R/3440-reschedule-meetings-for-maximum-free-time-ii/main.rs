impl Solution {
  pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let mut interval: Vec<(i32, usize)> = vec![];
    interval.push((start_time[0], 0));
    for i in 0..start_time.len() - 1 {
      interval.push((start_time[i + 1] - end_time[i], i + 1));
    }
    interval.push((event_time - end_time[end_time.len() - 1], end_time.len()));

    let mut intl_clone = interval.clone();
    intl_clone.sort_unstable();
    // println!("{:?}", interval);

    let mut ans: i32 = 0;
    for i in 0..start_time.len() {
      ans = ans.max(interval[i].0 + interval[i + 1].0);
      let diff = end_time[i] - start_time[i];
      let pos = intl_clone.partition_point(|x| x.0 < diff);
      match intl_clone.len() - pos {
        0 => {}
        1 => {
          if intl_clone[pos].1 != i && intl_clone[pos].1 != i + 1 {
            ans = ans.max(interval[i].0 + interval[i + 1].0 + diff);
          }
        }
        2 => {
          if intl_clone[pos].1 != i && intl_clone[pos].1 != i + 1
            || intl_clone[pos + 1].1 != i && intl_clone[pos + 1].1 != i + 1
          {
            ans = ans.max(interval[i].0 + interval[i + 1].0 + diff);
          }
        }
        _ => {
          ans = ans.max(interval[i].0 + interval[i + 1].0 + diff);
        }
      };
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_free_time(5, vec![1, 3], vec![2, 5]));
}
