struct Solution {}

impl Solution {
  pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
    let mut pg_time = plant_time
      .into_iter()
      .zip(grow_time.into_iter())
      .collect::<Vec<(i32, i32)>>();
    pg_time.sort_by(|x, y| x.1.cmp(&y.1).reverse());
    // println!("{:?}", pg_time);
    let mut sum : i32 = 0;
    let mut prev: i32 = 0;
    for (p, g) in pg_time {
      sum = sum.max(prev + p + g);
      prev += p;
    }
    sum
  }
}

fn main() {
  println!(
    "{}",
    Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1])
  );
}
