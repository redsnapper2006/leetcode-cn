struct Solution {}

impl Solution {
  pub fn rampart_defensive_line(rampart: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    let mut min: i32 = 1 << 31 - 1;
    (0..rampart.len() - 1).for_each(|idx| {
      let diff = rampart[idx + 1][0] - rampart[idx][1];
      if min > diff {
        min = diff;
      }
      sum += diff;
      buf.push(diff);
    });
    let mut max = sum / (rampart.len() as i32 - 2);

    let mut dis = min;
    while dis <= max {
      // println!("dist {} max {}", dis, max);
      let m = dis + (max - dis) / 2;
      let mut valid: bool = true;

      let mut buf2 = buf.clone();
      let mut idx: usize = 0;
      // println!("{:?}", buf2);
      while idx < buf2.len() - 1 {
        // println!("buf {} {} {} {}", idx, buf2[idx], buf2[idx + 1], m);
        if buf2[idx] + buf2[idx + 1] < m {
          valid = false;
          break;
        }
        let mut sub = buf2[idx];
        if sub > m {
          sub = m;
        }
        let remain = m - sub;
        buf2[idx + 1] -= remain;

        idx += 1;
      }

      if valid {
        dis = m + 1;
      } else {
        max = m - 1;
      }
    }

    dis - 1
  }
}
fn main() {
  println!(
    "{}",
    Solution::rampart_defensive_line(vec![vec![0, 3], vec![4, 5], vec![7, 9]])
  );

  println!(
    "{}",
    Solution::rampart_defensive_line(vec![
      vec![3, 5],
      vec![12, 29],
      vec![31, 38],
      vec![39, 42],
      vec![43, 44],
      vec![46, 47]
    ])
  );

  println!(
    "{}",
    Solution::rampart_defensive_line(vec![vec![1, 2], vec![5, 8], vec![11, 15], vec![18, 25]])
  );
}
