use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  pub fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
  ) -> Vec<String> {
    let mut sup: HashMap<String, bool> = HashMap::new();
    for i in 0..supplies.len() {
      sup.insert(supplies[i].clone(), true);
    }
    let mut idx2rec: HashMap<String, usize> = HashMap::new();
    for i in 0..recipes.len() {
      idx2rec.insert(recipes[i].clone(), i);
    }

    fn dfs(
      parent: &mut HashSet<String>,
      ing: String,
      idx: usize,
      ingre: &Vec<Vec<String>>,
      idx2rec: &HashMap<String, usize>,
      sup: &mut HashMap<String, bool>,
    ) -> bool {
      let mut possible: bool = true;
      for i in 0..ingre[idx].len() {
        if parent.contains(&ingre[idx][i]) {
          possible = false;
          break;
        }
        if sup.contains_key(&ingre[idx][i]) {
          if !sup.get(&ingre[idx][i]).unwrap() {
            possible = false;
            break;
          }
        } else {
          parent.insert(ingre[idx][i].clone());
          if !idx2rec.contains_key(&ingre[idx][i])
            || !dfs(
              parent,
              ingre[idx][i].clone(),
              *idx2rec.get(&ingre[idx][i]).unwrap(),
              ingre,
              idx2rec,
              sup,
            )
          {
            possible = false;
            break;
          }
          parent.remove(&ingre[idx][i]);
        }
      }
      sup.insert(ing, possible);
      possible
    }

    let mut ans: Vec<String> = vec![];
    for i in 0..recipes.len() {
      let mut parent: HashSet<String> = HashSet::new();
      parent.insert(recipes[i].clone());
      let possible: bool = dfs(
        &mut parent,
        recipes[i].clone(),
        i,
        &ingredients,
        &idx2rec,
        &mut sup,
      );
      if possible {
        ans.push(recipes[i].clone());
      }
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::find_all_recipes(
      vec![
        "burger".to_string(),
        "bread".to_string(),
        "sandwich".to_string()
      ],
      vec![
        vec![
          "sandwich".to_string(),
          "meat".to_string(),
          "bread".to_string()
        ],
        vec!["yeast".to_string(), "flour".to_string()],
        vec!["bread".to_string(), "meat".to_string()]
      ],
      vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()]
    )
  );

  // println!(
  //   "{:?}",
  //   Solution::find_all_recipes(
  //     vec![
  //       "ju".to_string(),
  //       "fzjnm".to_string(),
  //       "x".to_string(),
  //       "e".to_string(),
  //       "zpmcz".to_string(),
  //       "h".to_string(),
  //       "q".to_string()
  //     ],
  //     vec![
  //       vec!["d".to_string()],
  //       vec!["hveml".to_string(), "f".to_string(), "cpivl".to_string()],
  //       vec![
  //         "cpivl".to_string(),
  //         "zpmcz".to_string(),
  //         "h".to_string(),
  //         "e".to_string(),
  //         "fzjnm".to_string(),
  //         "ju".to_string(),
  //       ],
  //       vec![
  //         "cpivl".to_string(),
  //         "hveml".to_string(),
  //         "zpmcz".to_string(),
  //         "ju".to_string(),
  //         "h".to_string()
  //       ],
  //       vec![
  //         "h".to_string(),
  //         "fzjnm".to_string(),
  //         "e".to_string(),
  //         "q".to_string(),
  //         "x".to_string(),
  //       ],
  //       vec![
  //         "d".to_string(),
  //         "hveml".to_string(),
  //         "cpivl".to_string(),
  //         "q".to_string(),
  //         "zpmcz".to_string(),
  //         "ju".to_string(),
  //         "e".to_string(),
  //         "x".to_string()
  //       ],
  //       vec!["f".to_string(), "hveml".to_string(), "cpivl".to_string()]
  //     ],
  //     vec![
  //       "f".to_string(),
  //       "hveml".to_string(),
  //       "cpivl".to_string(),
  //       "d".to_string()
  //     ]
  //   )
  // );
}
