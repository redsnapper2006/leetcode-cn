impl Solution {
  pub fn earliest_finish_time(
    land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>,
  ) -> i32 {
    fn search(
      first_start_time: &Vec<i32>, first_duration: &Vec<i32>, second_start_time: &Vec<i32>, second_duration: &Vec<i32>,
    ) -> i32 {
      let mut end1: i32 = i32::MAX;
      for i in 0..first_start_time.len() {
        end1 = end1.min(first_start_time[i] + first_duration[i]);
      }
      let mut end2: i32 = i32::MAX;
      for i in 0..second_start_time.len() {
        end2 = end2.min(second_start_time[i].max(end1) + second_duration[i]);
      }
      end2
    }
    search(&land_start_time, &land_duration, &water_start_time, &water_duration).min(search(
      &water_start_time,
      &water_duration,
      &land_start_time,
      &land_duration,
    ))
  }

  pub fn earliest_finish_time2(
    land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>,
  ) -> i32 {
    fn merge(start_time: &Vec<i32>, duration: &Vec<i32>) -> (Vec<(i32, i32)>, Vec<i32>) {
      let mut sd: Vec<(i32, i32)> = vec![];
      let mut s: Vec<i32> = vec![];
      for i in 0..start_time.len() {
        sd.push((start_time[i], duration[i]));
        s.push(start_time[i] + duration[i]);
      }
      sd.sort_unstable();
      sd.dedup_by_key(|x| x.0);
      s.sort_unstable();
      s.dedup();
      (sd, s)
    }

    let (lsd, ls) = merge(&land_start_time, &land_duration);
    let (wsd, ws) = merge(&water_start_time, &water_duration);

    fn build(sd: &Vec<(i32, i32)>) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
      let mut left: Vec<(i32, i32)> = vec![];
      let mut base: i32 = i32::MAX;

      sd.iter().for_each(|&(s, d)| {
        base = base.min(d);
        left.push((s, base));
      });

      let mut right: Vec<(i32, i32)> = vec![];
      let mut base: i32 = i32::MAX;
      sd.iter().rev().for_each(|&(s, d)| {
        base = base.min(s + d);
        right.push((s, base));
      });
      right.reverse();
      (left, right)
    }

    let mut ans: i32 = i32::MAX;
    let (ll, lr) = build(&lsd);
    let (wl, wr) = build(&wsd);

    fn search(sd: &Vec<i32>, l: &Vec<(i32, i32)>, r: &Vec<(i32, i32)>, ans: &mut i32) {
      let mut idx: usize = 0;
      for i in 0..sd.len() {
        while idx < l.len() && l[idx].0 <= sd[i] {
          idx += 1;
        }

        if idx > 0 {
          (*ans) = (*ans).min(sd[i].max(l[idx - 1].0) + l[idx - 1].1);
        }

        if idx < r.len() {
          (*ans) = (*ans).min(r[idx].1);
        }
      }
    }

    search(&ls, &wl, &wr, &mut ans);
    search(&ws, &ll, &lr, &mut ans);

    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]));
  println!("{}", Solution::earliest_finish_time(vec![99], vec![59], vec![99, 54], vec![85, 20]));
  println!("{}", Solution::earliest_finish_time(vec![76, 67], vec![9, 72], vec![86, 90], vec![10, 12]));
  println!("{}", Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]));
  println!("{}", Solution::earliest_finish_time(vec![100000], vec![100000], vec![100000], vec![100000]));
}
