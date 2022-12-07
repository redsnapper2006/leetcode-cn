impl Solution {
  pub fn best_closing_time(customers: String) -> i32 {
    let mut close: Vec<i32> = customers
      .as_bytes()
      .iter()
      .map(|w| if *w == 'Y' as u8 { 1 } else { 0 })
      .collect();
    let mut open: Vec<i32> = customers
      .as_bytes()
      .iter()
      .map(|w| if *w == 'N' as u8 { 1 } else { 0 })
      .collect();

    for i in 1..open.len() {
      open[i] += open[i - 1];
    }
    for i in (0..close.len() - 1).rev() {
      close[i] += close[i + 1];
    }

    let mut ret: i32 = customers.len() as i32;
    let mut idx: i32 = 0;
    for i in 0..=open.len() {
      let mut o: i32 = 0;
      if i > 0 {
        o = open[i - 1];
      }
      let mut c: i32 = 0;
      if i < open.len() {
        c = close[i];
      }

      if o + c < ret {
        idx = i as i32;
        ret = o + c;
      }
    }
    idx
  }
}
