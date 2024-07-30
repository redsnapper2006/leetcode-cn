struct Solution {}

impl Solution {
  pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    operations.iter().for_each(|oper| {
      match oper.as_str() {
        "C" => {
          stack.pop();
        }
        "D" => stack.push(stack[stack.len() - 1] * 2),
        "+" => stack.push(stack[stack.len() - 1] + stack[stack.len() - 2]),
        _ => stack.push(oper.parse::<i32>().unwrap_or(0)),
      };
    });
    stack.iter().sum()
  }
}

fn main() {
  println!(
    "{}",
    Solution::cal_points(vec![
      "5".to_string(),
      "-2".to_string(),
      "4".to_string(),
      "C".to_string(),
      "D".to_string(),
      "9".to_string(),
      "+".to_string(),
      "+".to_string()
    ])
  );
}
