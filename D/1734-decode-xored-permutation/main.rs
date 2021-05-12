struct Solution {}

impl Solution {
  pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
    let mut xor_aggr: i32 = 0;
    for i in 1..=encoded.len() + 1 {
      xor_aggr = xor_aggr ^ i as i32;
    }
    let mut even_aggr: i32 = 0;
    for i in (1..encoded.len()).step_by(2) {
      even_aggr = even_aggr ^ encoded[i];
    }
    let mut ret: Vec<i32> = vec![0; encoded.len() + 1];
    ret[0] = xor_aggr ^ even_aggr;
    for i in 1..=encoded.len() {
      ret[i] = ret[i - 1] ^ encoded[i - 1];
    }
    ret
  }
}

fn main() {
  println!("{:?}", Solution::decode(vec![6, 5, 4, 6]));
}
