impl Solution {
  pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut bits: Vec<i32> = vec![0; 10001];
    (1..10001).for_each(|idx| {
      bits[idx] = bits[idx >> 1] + (idx as i32 & 1);
    });

    let mut arr = arr;
    arr.sort_by(|&x, &y| {
      if bits[x as usize] == bits[y as usize] {
        x.cmp(&y)
      } else {
        bits[x as usize].cmp(&bits[y as usize])
      }
    });
    arr
  }
}
