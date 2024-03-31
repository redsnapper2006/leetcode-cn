struct Solution {}

impl Solution {
  pub fn is_valid_serialization(preorder: String) -> bool {
    let pa: Vec<&str> = preorder.split(",").collect::<Vec<&str>>();

    let mut idx: usize = 0;
    let mut slot: i32 = 1;
    while idx < pa.len() {
      if slot == 0 {
        return false;
      }

      if pa[idx] == "#" {
        slot -= 1;
      } else {
        slot += 1;
      }

      idx += 1;
    }

    slot == 0
  }
}

fn main() {
  println!("{}", Solution::is_valid_serialization("1,2,3".to_string()));
}
