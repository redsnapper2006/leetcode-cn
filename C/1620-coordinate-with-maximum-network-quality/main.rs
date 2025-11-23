use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
    let mut dis: HashMap<(i32, i32), i32> = HashMap::new();

    towers.iter().for_each(|tw| {
      let x = tw[0];
      let y = tw[1];
      let p = tw[2];
      for ix in 0..=radius {
        let yr = ((radius * radius - ix * ix) as f64).sqrt() as i32;
        for iy in 0..=yr {
          let v = (p as f64 / (1.0 + ((ix * ix + iy * iy) as f64).sqrt())) as i32;

          let mut cord: HashSet<(i32, i32)> = HashSet::new();
          cord.insert((x + ix, y + iy));
          cord.insert((x - ix, y + iy));
          cord.insert((x + ix, y - iy));
          cord.insert((x - ix, y - iy));

          for &(nx, ny) in cord.iter() {
            if nx < 0 || ny < 0 {
              continue;
            }
            dis.entry((nx, ny)).and_modify(|x| *x += v).or_insert(v);
          }
        }
      }
    });

    let mut max: i32 = -1;
    let mut ansx: i32 = 0;
    let mut ansy: i32 = 0;
    for (&(x, y), &v) in dis.iter() {
      if v > max {
        ansx = x;
        ansy = y;
        max = v;
      } else if v == max && (x < ansx || x == ansx && y < ansy) {
        ansx = x;
        ansy = y;
      }
    }

    if max == 0 {
      vec![0, 0]
    } else {
      vec![ansx, ansy]
    }
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2)
  );
  println!(
    "{:?}",
    Solution::best_coordinate(vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]], 2)
  );
  println!(
    "{:?}",
    Solution::best_coordinate(
      vec![vec![0, 1, 2], vec![2, 1, 2], vec![1, 0, 2], vec![1, 2, 2]],
      1
    )
  );
  println!(
    "{:?}",
    Solution::best_coordinate(
      vec![vec![18, 12, 31], vec![45, 39, 36], vec![14, 26, 25]],
      34
    )
  );
  println!(
    "{:?}",
    Solution::best_coordinate(
      vec![
        vec![31, 13, 33],
        vec![24, 45, 38],
        vec![28, 32, 23],
        vec![7, 23, 22],
        vec![41, 50, 33],
        vec![47, 21, 3],
        vec![3, 33, 39],
        vec![11, 38, 5],
        vec![26, 20, 28],
        vec![48, 39, 16],
        vec![34, 29, 25]
      ],
      22
    )
  );
}
