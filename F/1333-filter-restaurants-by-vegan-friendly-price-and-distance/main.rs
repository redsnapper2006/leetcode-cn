struct Solution {}

impl Solution {
  pub fn filter_restaurants(
    restaurants: Vec<Vec<i32>>,
    vegan_friendly: i32,
    max_price: i32,
    max_distance: i32,
  ) -> Vec<i32> {
    let mut restaurants = restaurants;
    restaurants.sort_by(|x, y| {
      if x[1] == y[1] {
        return x[0].cmp(&y[0]).reverse();
      }
      x[1].cmp(&y[1]).reverse()
    });

    restaurants
      .iter()
      .filter(|x| {
        if vegan_friendly == 1 && x[2] == 0 {
          return false;
        }
        if x[3] > max_price {
          return false;
        }
        if x[4] > max_distance {
          return false;
        }
        return true;
      })
      .map(|x| x[0])
      .collect::<Vec<i32>>()
  }
}
