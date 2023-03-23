struct Solution {}

impl Solution {
  pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let n = scores.len();
    let mut myzip: Vec<(i32, i32)> = ages
      .into_iter()
      .zip(scores.into_iter())
      .collect::<Vec<(i32, i32)>>();

    myzip.sort_by(|x, y| {
      let xa = &x.0;
      let ya = &y.0;
      let xs = &x.1;
      let ys = &y.1;
      if xa != ya {
        return xa.cmp(ya);
      }
      xs.cmp(ys)
    });

    let mut dp: Vec<i32> = vec![0; n];
    dp[0] = myzip[0].1;
    (0..myzip.len()).for_each(|i| {
      let mut d: i32 = myzip[i].1;
      (0..i).rev().for_each(|j| {
        if myzip[i].1 >= myzip[j].1 {
          d = d.max(myzip[i].1 + dp[j]);
        }
      });
      dp[i] = d;
    });
    *dp.iter().max().unwrap()
  }

  pub fn best_team_score2(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut cached: HashMap<usize, i32> = HashMap::new();
    let mut myzip: Vec<(i32, i32)> = ages
      .into_iter()
      .zip(scores.into_iter())
      .collect::<Vec<(i32, i32)>>();

    myzip.sort_by(|x, y| {
      let xa = &x.0;
      let ya = &y.0;
      let xs = &x.1;
      let ys = &y.1;
      if xa != ya {
        return xa.cmp(ya);
      }
      xs.cmp(ys)
    });
    // println!("{:?}", myzip);

    fn dfs(
      myzip: &Vec<(i32, i32)>,
      idx: usize,
      age: i32,
      score: i32,
      sum: i32,
      cached: &mut HashMap<usize, i32>,
    ) {
      let v = cached.entry(idx).or_insert(0);
      if sum > *v {
        *v = sum;
      } else {
        return;
      }
      for i in idx + 1..myzip.len() {
        if myzip[i].0 > age && myzip[i].1 < score {
          continue;
        }
        dfs(myzip, i, myzip[i].0, myzip[i].1, sum + myzip[i].1, cached);
      }
    }

    for i in 0..myzip.len() {
      dfs(&myzip, i, myzip[i].0, myzip[i].1, myzip[i].1, &mut cached);
    }

    *cached.values().max().unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5])
  );
}
