use std::collections::{BinaryHeap, HashMap};

struct AuctionSystem {
  // item->(bid, user)
  mq: HashMap<i32, BinaryHeap<(i32, i32)>>,
  // item->user->bid
  h: HashMap<i32, HashMap<i32, i32>>,
}

impl AuctionSystem {
  fn new() -> Self {
    AuctionSystem {
      mq: HashMap::new(),
      h: HashMap::new(),
    }
  }

  fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
    self.h.entry(item_id).or_insert(HashMap::new()).insert(user_id, bid_amount);
    self.mq.entry(item_id).or_insert(BinaryHeap::new()).push((bid_amount, user_id));
  }

  fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
    self.add_bid(user_id, item_id, new_amount);
  }

  fn remove_bid(&mut self, user_id: i32, item_id: i32) {
    self.h.get_mut(&item_id).unwrap().remove(&user_id);
  }

  fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
    if !self.h.contains_key(&item_id) {
      return -1;
    }
    let heap = self.mq.get_mut(&item_id).unwrap();
    let ub = self.h.get(&item_id).unwrap();

    while let Some(&(bid, user)) = heap.peek() {
      if !ub.contains_key(&user) || matches!(ub.get(&user), Some(b) if *b != bid) {
        heap.pop();
      } else {
        return user;
      }
    }
    -1
  }
}
