struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut m: HashMap<String, i32> = HashMap::new();
    let mut ret: Vec<String> = Vec::new();
    for n in &names {
      let cnt = m.entry((*n).to_string()).or_insert(0);
      if *cnt == 0 {
        *cnt = 1;
        ret.push((*n).to_string());
      } else {
        let mut c = *cnt;
        let mut nn = format!("{}({})", *n, c);

        while m.contains_key(&nn) {
          c += 1;
          nn = format!("{}({})", *n, c).to_string();
        }
        m.insert(nn.clone(), 1);
        m.insert((*n).to_string(), c + 1);
        ret.push(nn);
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::get_folder_names(vec![
      String::from("pes"),
      String::from("fifa"),
      String::from("gta"),
      String::from("pes(2019)")
    ])
  );
  println!(
    "{:?}",
    Solution::get_folder_names(vec![
      String::from("gta"),
      String::from("gta(1)"),
      String::from("gta"),
      String::from("avalon")
    ])
  );
  println!(
    "{:?}",
    Solution::get_folder_names(vec![
      String::from("onepiece"),
      String::from("onepiece(1)"),
      String::from("onepiece(2)"),
      String::from("onepiece(3)"),
      String::from("onepiece")
    ])
  );
  println!(
    "{:?}",
    Solution::get_folder_names(vec![
      String::from("wano"),
      String::from("wano"),
      String::from("wano"),
      String::from("wano")
    ])
  );
  println!(
    "{:?}",
    Solution::get_folder_names(vec![
      String::from("kaido"),
      String::from("kaido(1)"),
      String::from("kaido"),
      String::from("kaido(1)")
    ])
  );
}
