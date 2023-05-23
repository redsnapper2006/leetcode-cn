struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn largest_vals_from_labels(
    values: Vec<i32>,
    labels: Vec<i32>,
    num_wanted: i32,
    use_limit: i32,
  ) -> i32 {
    let mut mvl: HashMap<i32, Vec<i32>> = HashMap::new();
    let zz: Vec<(i32, i32)> = values
      .into_iter()
      .zip(labels.into_iter())
      .collect::<Vec<(i32, i32)>>();
    zz.iter().for_each(|(v, l)| {
      let vls = mvl.entry(*l).or_insert(vec![]);
      vls.push(*v);
    });
    mvl.iter_mut().for_each(|(_, v)| {
      v.sort();
      v.reverse();
    });
    // println!("{:?}", mvl);
    let mut used: HashMap<i32, i32> = HashMap::new();
    let mut sum: i32 = 0;
    let mut idx: i32 = 0;
    while idx < num_wanted {
      let mut candi_key: i32 = -1;
      let mut max: i32 = 0;
      mvl.iter().for_each(|(k, v)| {
        let t = v.get(0).unwrap();
        if max < *t {
          max = *t;
          candi_key = *k;
        }
      });
      if candi_key == -1 {
        break;
      }
      // println!("candi_key: {} max {}", candi_key, max);
      sum += max;
      let candi = mvl.get_mut(&candi_key).unwrap();
      candi.remove(0);

      let cnt = used.entry(candi_key).or_insert(0);
      *cnt += 1;
      if candi.len() == 0 || *cnt == use_limit {
        mvl.remove(&candi_key);
      }
      idx += 1;
    }
    sum
  }
}

fn main() {
  println!(
    "{}",
    Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1)
  );

  println!(
    "{}",
    Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2)
  );
}
