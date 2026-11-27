impl Solution {
  pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
    let budget = budget as usize;
    let mut mn = i32::MAX;
    let mut price_cnt: Vec<(usize, i32)> = vec![];
    for i in 0..items.len() {
      mn = mn.min(items[i][1]);
      let mut cnt = 0i32;
      for j in 0..items.len() {
        if items[j][0] < items[i][0] {
          continue;
        }
        if items[j][0] % items[i][0] == 0 {
          cnt += 1;
        }
      }
      price_cnt.push((items[i][1] as usize, cnt));
    }

    let mut dp: Vec<i32> = vec![0; budget + 1];
    for &(price, cnt) in &price_cnt {
      if price > budget {
        continue;
      }
      for i in (price..=budget).rev() {
        dp[i] = dp[i].max(dp[i - price] + cnt);
      }
    }

    (0..=budget).fold(0, |ans, idx| ans.max(dp[idx] + (budget - idx) as i32 / mn))
  }
}
