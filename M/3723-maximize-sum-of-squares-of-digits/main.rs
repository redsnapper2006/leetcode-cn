impl Solution {
  pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
    if sum > num * 9 {
      return "".to_string();
    }
    let mut ans: Vec<u8> = vec![b'9'; sum as usize / 9];
    if sum % 9 > 0 {
      ans.push(b'0' + (sum % 9) as u8);
    }
    ans.append(&mut vec![b'0'; num as usize - ans.len()]);
    String::from_utf8(ans).unwrap()
  }
}
