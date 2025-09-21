use std::collections::{BTreeMap, BTreeSet, HashMap};

struct MovieRentingSystem {
  m: HashMap<i32, BTreeMap<i32, BTreeSet<i32>>>, // movie -> price->shop
  r: BTreeSet<(i32, i32, i32)>,                  // (price, shop, movie)
  sm: HashMap<(i32, i32), i32>,                  // (shop, movie)->price
}

impl MovieRentingSystem {
  // shop, movie, price
  fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
    let mut sm: HashMap<(i32, i32), i32> = HashMap::new();
    let mut m: HashMap<i32, BTreeMap<i32, BTreeSet<i32>>> = HashMap::new();
    entries.iter().for_each(|entry| {
      sm.insert((entry[0], entry[1]), entry[2]);
      m.entry(entry[1])
        .and_modify(|x| {
          x.entry(entry[2])
            .and_modify(|y| {
              y.insert(entry[0]);
            })
            .or_insert(BTreeSet::from([entry[0]]));
        })
        .or_insert(BTreeMap::from([(entry[2], BTreeSet::from([entry[0]]))]));
    });
    MovieRentingSystem {
      m: m,
      r: BTreeSet::new(),
      sm: sm,
    }
  }

  fn search(&self, movie: i32) -> Vec<i32> {
    if !self.m.contains_key(&movie) {
      return vec![];
    }
    let btm = self.m.get(&movie).unwrap();
    let mut ans: Vec<i32> = vec![];
    let mut is_enough: bool = false;
    for (p, ts) in btm.iter() {
      for s in ts.iter() {
        if self.r.contains(&(*p, *s, movie)) {
          continue;
        }
        ans.push(*s);
        if ans.len() == 5 {
          is_enough = true;
          break;
        }
      }
      if is_enough {
        break;
      }
    }
    ans
  }

  fn rent(&mut self, shop: i32, movie: i32) {
    let p = self.sm.get(&(shop, movie)).unwrap();
    self.r.insert((*p, shop, movie));
  }

  fn drop(&mut self, shop: i32, movie: i32) {
    let p = self.sm.get(&(shop, movie)).unwrap();
    self.r.remove(&(*p, shop, movie));
  }

  fn report(&self) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    for (_, s, m) in self.r.iter() {
      ans.push(vec![*s, *m]);
      if ans.len() == 5 {
        break;
      }
    }
    ans
  }
}

fn main() {
  let mut mrs = MovieRentingSystem::new(
    10,
    vec![
      vec![4, 374, 55],
      vec![1, 6371, 21],
      vec![8, 3660, 24],
      vec![1, 56, 32],
      vec![5, 374, 71],
      vec![3, 4408, 36],
      vec![6, 9322, 73],
      vec![6, 9574, 92],
      vec![8, 7834, 62],
      vec![2, 6084, 27],
      vec![7, 3262, 89],
      vec![2, 8959, 53],
      vec![0, 3323, 41],
      vec![6, 6565, 45],
      vec![0, 4239, 20],
    ],
  );
  mrs.rent(0, 4239);
  mrs.drop(0, 4239);
  mrs.rent(3, 4408);
  mrs.rent(2, 6084);
  mrs.rent(0, 4239);
  mrs.drop(0, 4239);
  println!("{:?}", mrs.search(9346));
  println!("report {:?}", mrs.report());
  mrs.rent(6, 9322);
  println!("{:?}", mrs.search(8698));
}
