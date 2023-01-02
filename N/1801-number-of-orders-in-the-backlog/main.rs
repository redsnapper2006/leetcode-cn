struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
impl Solution {
  pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
    let mut sell: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut buy: BinaryHeap<i32> = BinaryHeap::new();
    let mut samount: HashMap<i32, i32> = HashMap::new();
    let mut bamount: HashMap<i32, i32> = HashMap::new();

    for order in orders {
      let ot = order[2];
      match ot {
        0 => {
          let mut amount = order[1];
          while amount > 0 && sell.len() > 0 && sell.peek().unwrap().0 <= order[0] {
            let mut sa = samount.get_mut(&sell.peek().unwrap().0).unwrap();
            let mut diff = *sa;
            if diff > amount {
              diff = amount;
            }
            *sa -= diff;
            amount -= diff;
            if *sa == 0 {
              samount.remove(&sell.peek().unwrap().0);
              sell.pop();
            }
          }
          if amount > 0 {
            if bamount.contains_key(&order[0]) {
              let mut v = bamount.get_mut(&order[0]).unwrap();
              *v += amount;
            } else {
              buy.push(order[0]);
              bamount.insert(order[0], amount);
            }
          }
        }
        1 => {
          let mut amount = order[1];
          while amount > 0 && buy.len() > 0 && *buy.peek().unwrap() >= order[0] {
            let mut ba = bamount.get_mut(buy.peek().unwrap()).unwrap();
            let mut diff = *ba;
            if diff > amount {
              diff = amount;
            }
            *ba -= diff;
            amount -= diff;
            if *ba == 0 {
              bamount.remove(buy.peek().unwrap());
              buy.pop();
            }
          }
          if amount > 0 {
            if samount.contains_key(&order[0]) {
              let mut v = samount.get_mut(&order[0]).unwrap();
              *v += amount;
            } else {
              sell.push(Reverse(order[0]));
              samount.insert(order[0], amount);
            }
          }
        }
        _ => {}
      }
    }

    let mut ret: i64 = 0;
    for (_, mut v) in samount {
      v %= 1000000007;
      ret += v as i64;
      ret %= 1000000007
    }
    for (_, mut v) in bamount {
      v %= 1000000007;
      ret += v as i64;
      ret %= 1000000007
    }
    ret as i32
  }
}
