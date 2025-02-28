use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct CRF {
  r: i32,
  f: String,
}

impl Ord for CRF {
  fn cmp(&self, other: &Self) -> Ordering {
    self.r.cmp(&other.r).then_with(|| other.f.cmp(&self.f))
  }
}

impl PartialOrd for CRF {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

struct FoodRatings {
  fr: HashMap<String, i32>,
  fc: HashMap<String, String>,
  crbhf: HashMap<String, BinaryHeap<CRF>>,
}

impl FoodRatings {
  fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
    let mut fr: HashMap<String, i32> = HashMap::new();
    let mut fc: HashMap<String, String> = HashMap::new();
    let mut crbhf: HashMap<String, BinaryHeap<CRF>> = HashMap::new();
    (0..foods.len()).for_each(|idx| {
      fr.insert(foods[idx].clone(), ratings[idx]);
      fc.insert(foods[idx].clone(), cuisines[idx].clone());
      crbhf
        .entry(cuisines[idx].clone())
        .and_modify(|x| {
          x.push(CRF {
            r: ratings[idx].clone(),
            f: foods[idx].clone(),
          })
        })
        .or_insert(BinaryHeap::from([CRF {
          r: ratings[idx],
          f: foods[idx].clone(),
        }]));
    });
    FoodRatings {
      fr: fr,
      fc: fc,
      crbhf: crbhf,
    }
  }

  fn change_rating(&mut self, food: String, new_rating: i32) {
    self.fr.insert(food.clone(), new_rating);
    let c = self.fc.get(&food).unwrap();
    self.crbhf.get_mut(c).unwrap().push(CRF {
      r: new_rating,
      f: food.clone(),
    });
  }

  fn highest_rated(&mut self, cuisine: String) -> String {
    let cr = self.crbhf.get_mut(&cuisine).unwrap();
    while cr.len() > 0 {
      let crf = cr.peek().unwrap();
      if *self.fr.get(&crf.f).unwrap() != crf.r {
        cr.pop();
      } else {
        return crf.f.clone();
      }
    }
    "".to_string()
  }
}

fn main() {
  let mut fr = FoodRatings::new(
    vec![
      "kimchi".to_string(),
      "miso".to_string(),
      "sushi".to_string(),
      "moussaka".to_string(),
      "ramen".to_string(),
      "bulgogi".to_string(),
    ],
    vec![
      "korean".to_string(),
      "japanese".to_string(),
      "japanese".to_string(),
      "greek".to_string(),
      "japanese".to_string(),
      "korean".to_string(),
    ],
    vec![9, 12, 8, 15, 14, 7],
  );

  println!("{}", fr.highest_rated("korean".to_string()));
  println!("{}", fr.highest_rated("japanese".to_string()));
  fr.change_rating("sushi".to_string(), 16);
  println!("{}", fr.highest_rated("japanese".to_string()));
  fr.change_rating("ramen".to_string(), 16);
  println!("{}", fr.highest_rated("japanese".to_string()));
}
