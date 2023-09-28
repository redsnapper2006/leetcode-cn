struct Solution {}

impl Solution {
  pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    let mut buf: Vec<i32> = Vec::new();
    flowers.iter().for_each(|flower| {
      buf.push(flower[0]);
      buf.push(flower[1] + 1);
    });
    buf.sort();
    buf.dedup();

    let mut aggr: Vec<i32> = vec![0; buf.len()];
    flowers.iter().for_each(|flower| {
      // println!("{}", flower[1]);
      let offset1 = buf.binary_search(&flower[0]).unwrap();
      let offset2 = buf.binary_search(&(flower[1] + 1)).unwrap();
      aggr[offset1] += 1;
      aggr[offset2] -= 1;
    });

    let mut sum: i32 = 0;
    (0..buf.len()).for_each(|i| {
      sum += aggr[i];
      aggr[i] = sum;
    });

    people
      .iter()
      .map(|person| {
        // println!("{}", person);
        match buf.binary_search(person) {
          Ok(offset) => aggr[offset],
          Err(offset) => match offset {
            0 => 0,
            _ => aggr[offset - 1],
          },
        }
      })
      .collect()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::full_bloom_flowers(
      vec![vec![11, 11], vec![24, 46], vec![3, 25], vec![44, 46]],
      vec![1, 8, 26, 7, 43, 26, 1]
    )
  );

  println!(
    "{:?}",
    Solution::full_bloom_flowers(
      vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
      vec![2, 3, 7, 11]
    )
  );
  println!(
    "{:?}",
    Solution::full_bloom_flowers(vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2])
  );
  println!(
    "{:?}",
    Solution::full_bloom_flowers(
      vec![vec![19, 37], vec![19, 38], vec![19, 35]],
      vec![6, 7, 21, 1, 13, 37, 5, 37, 46, 43]
    )
  );
}
