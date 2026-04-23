use std::collections::{HashSet, VecDeque};

struct RideSharingSystem {
  rider: VecDeque<i32>,
  driver: VecDeque<i32>,
  cancel: HashSet<i32>,
}

impl RideSharingSystem {
  fn new() -> Self {
    RideSharingSystem {
      rider: VecDeque::new(),
      driver: VecDeque::new(),
      cancel: HashSet::new(),
    }
  }

  fn add_rider(&mut self, rider_id: i32) {
    self.cancel.remove(&rider_id);
    self.rider.push_back(rider_id);
  }

  fn add_driver(&mut self, driver_id: i32) {
    self.driver.push_back(driver_id);
  }

  fn match_driver_with_rider(&mut self) -> Vec<i32> {
    if self.driver.len() == 0 {
      return vec![-1, -1];
    }

    let mut rider: i32 = 0;
    while self.rider.len() > 0 {
      let t = self.rider.pop_front().unwrap();
      if !self.cancel.contains(&t) {
        rider = t;
        break;
      }
    }
    if rider == 0 {
      vec![-1, -1]
    } else {
      vec![self.driver.pop_front().unwrap(), rider]
    }
  }

  fn cancel_rider(&mut self, rider_id: i32) {
    self.cancel.insert(rider_id);
  }
}
