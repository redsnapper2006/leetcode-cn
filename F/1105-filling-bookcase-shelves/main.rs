struct Solution {}

impl Solution {
  pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut dp: Vec<i32> = vec![1000000; books.len() + 1];
    dp[0] = 0;
    (0..books.len()).for_each(|idx| match idx {
      0 => {
        dp[idx + 1] = dp[idx] + books[idx][1];
      }
      _ => {
        let mut start: usize = idx;
        let mut max: i32 = 0;
        let mut sum: i32 = 0;
        let mut total: i32 = 1000000;
        while start >= 0 {
          sum += books[start][0];
          if sum > shelf_width {
            break;
          }
          if max < books[start][1] {
            max = books[start][1];
          }
          if total > dp[start] + max {
            total = dp[start] + max;
          }
          if start == 0 {
            break;
          }
          start -= 1;
        }
        dp[idx + 1] = total;
      }
    });
    dp[books.len()]
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_height_shelves(
      vec![
        vec![1, 1],
        vec![2, 3],
        vec![2, 3],
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
        vec![1, 2]
      ],
      4
    )
  );

  println!(
    "{}",
    Solution::min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6)
  );

  println!(
    "{}",
    Solution::min_height_shelves(
      vec![
        vec![11, 83],
        vec![170, 4],
        vec![93, 80],
        vec![155, 163],
        vec![134, 118],
        vec![75, 14],
        vec![122, 192],
        vec![123, 154],
        vec![187, 29],
        vec![160, 64],
        vec![170, 152],
        vec![113, 179],
        vec![60, 102],
        vec![28, 187],
        vec![59, 95],
        vec![187, 97],
        vec![49, 193],
        vec![67, 126],
        vec![75, 45],
        vec![130, 160],
        vec![4, 102],
        vec![116, 171],
        vec![43, 170],
        vec![96, 188],
        vec![54, 15],
        vec![167, 183],
        vec![58, 158],
        vec![59, 55],
        vec![148, 183],
        vec![89, 95],
        vec![90, 113],
        vec![51, 49],
        vec![91, 28],
        vec![172, 103],
        vec![173, 3],
        vec![131, 78],
        vec![11, 199],
        vec![77, 200],
        vec![58, 65],
        vec![77, 30],
        vec![157, 58],
        vec![18, 194],
        vec![101, 148],
        vec![22, 197],
        vec![76, 181],
        vec![21, 176],
        vec![50, 45],
        vec![80, 174],
        vec![116, 198],
        vec![138, 9],
        vec![58, 125],
        vec![163, 102],
        vec![133, 175],
        vec![21, 39],
        vec![141, 156],
        vec![34, 185],
        vec![14, 113],
        vec![11, 34],
        vec![35, 184],
        vec![16, 132],
        vec![78, 147],
        vec![85, 170],
        vec![32, 149],
        vec![46, 94],
        vec![196, 3],
        vec![155, 90],
        vec![9, 114],
        vec![117, 119],
        vec![17, 157],
        vec![94, 178],
        vec![53, 55],
        vec![103, 142],
        vec![70, 121],
        vec![9, 141],
        vec![16, 170],
        vec![92, 137],
        vec![157, 30],
        vec![94, 82],
        vec![144, 149],
        vec![128, 160],
        vec![8, 147],
        vec![153, 198],
        vec![12, 22],
        vec![140, 68],
        vec![64, 172],
        vec![86, 63],
        vec![66, 158],
        vec![23, 15],
        vec![120, 99],
        vec![27, 165],
        vec![79, 174],
        vec![46, 19],
        vec![60, 98],
        vec![160, 172],
        vec![128, 184],
        vec![63, 172],
        vec![135, 54],
        vec![40, 4],
        vec![102, 171],
        vec![29, 125],
        vec![81, 9],
        vec![111, 197],
        vec![16, 90],
        vec![22, 150],
        vec![168, 126],
        vec![187, 61],
        vec![47, 190],
        vec![54, 110],
        vec![106, 102],
        vec![55, 47],
        vec![117, 134],
        vec![33, 107],
        vec![2, 10],
        vec![18, 62],
        vec![109, 188],
        vec![113, 37],
        vec![59, 159],
        vec![120, 175],
        vec![17, 147],
        vec![112, 195],
        vec![177, 53],
        vec![148, 173],
        vec![29, 105],
        vec![196, 32],
        vec![123, 51],
        vec![29, 19],
        vec![161, 178],
        vec![148, 2],
        vec![70, 124],
        vec![126, 9],
        vec![105, 87],
        vec![41, 121],
        vec![147, 10],
        vec![78, 167],
        vec![91, 197],
        vec![22, 98],
        vec![73, 33],
        vec![148, 194],
        vec![166, 64],
        vec![33, 138],
        vec![139, 158],
        vec![160, 19],
        vec![140, 27],
        vec![103, 109],
        vec![88, 16],
        vec![99, 181],
        vec![2, 140],
        vec![50, 188],
        vec![200, 77],
        vec![73, 84],
        vec![159, 130],
        vec![115, 199],
        vec![152, 79],
        vec![1, 172],
        vec![124, 136],
        vec![117, 138],
        vec![158, 86],
        vec![193, 150],
        vec![56, 57],
        vec![150, 133],
        vec![52, 186],
        vec![21, 145],
        vec![127, 97],
        vec![108, 110],
        vec![174, 44],
        vec![199, 169],
        vec![139, 200],
        vec![66, 48],
        vec![52, 190],
        vec![27, 86],
        vec![142, 191],
        vec![191, 79],
        vec![126, 114],
        vec![125, 100],
        vec![176, 95],
        vec![104, 79],
        vec![146, 189],
        vec![144, 78],
        vec![52, 106],
        vec![74, 74],
        vec![163, 128],
        vec![34, 181],
        vec![20, 178],
        vec![15, 107],
        vec![105, 8],
        vec![66, 142],
        vec![39, 126],
        vec![95, 59],
        vec![164, 69],
        vec![138, 18],
        vec![110, 145],
        vec![128, 200],
        vec![149, 150],
        vec![149, 93],
        vec![145, 140],
        vec![90, 170],
        vec![81, 127],
        vec![57, 151],
        vec![167, 127],
        vec![95, 89]
      ],
      200
    )
  );
}
