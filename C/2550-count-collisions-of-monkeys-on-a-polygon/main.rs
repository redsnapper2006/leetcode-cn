impl Solution {
  pub fn monkey_move(n: i32) -> i32 {
    let _mod: i64 = 1000000007;
    fn recur(n: i32, _mod: i64) -> i64 {
      if n == 1 {
        return 2;
      }
      let v = recur(n / 2, _mod);

      (((v * v) % _mod) * if n % 2 == 1 { 2 } else { 1 }) % _mod
    }

    ((recur(n, _mod) - 2 + _mod) % _mod) as i32
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::monkey_move(500000003));
}
