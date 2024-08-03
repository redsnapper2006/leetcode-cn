struct Solution {}

impl Solution {
  pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
    let mut letter_set: Vec<i32> = vec![0; 26];
    let mut cur_dist: i32 = -1;

    let mut dist = points
      .iter()
      .map(|point| point[0].abs().max(point[1].abs()))
      .zip(s.as_bytes().to_vec().into_iter())
      .collect::<Vec<(i32, u8)>>();
    dist.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", dist);
    let mut valid: bool = true;
    let mut ans: i32 = 0;
    let mut idx: usize = 0;
    let mut cnt: i32 = 0;
    while idx < dist.len() {
      let d = dist[idx];
      if d.0 > cur_dist {
        ans += cnt;
        cnt = 0;
      }
      cnt += 1;
      if letter_set[(d.1 - b'a') as usize] == 1 {
        valid = false;
        break;
      }

      cur_dist = d.0;
      letter_set[(d.1 - b'a') as usize] = 1;
      idx += 1;
    }
    if valid {
      ans += cnt;
    }
    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_points_inside_square(
      vec![
        vec![2, 2],
        vec![-1, -2],
        vec![-4, 4],
        vec![-3, 1],
        vec![3, -3]
      ],
      "abdca".to_string()
    )
  );
  println!(
    "{}",
    Solution::max_points_inside_square(
      vec![vec![1, 1], vec![-2, -2], vec![-2, 2]],
      "abb".to_string()
    )
  );
  println!(
    "{}",
    Solution::max_points_inside_square(
      vec![vec![1, 1], vec![-1, -1], vec![2, -2]],
      "ccd".to_string()
    )
  );
  // [[1,1],[-2,-2],[-2,2]]
  // "abb"
  // [[1,1],[-1,-1],[2,-2]]
  // "ccd"
}
