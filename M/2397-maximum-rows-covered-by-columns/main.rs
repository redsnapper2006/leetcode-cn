struct Solution {}

impl Solution {
  pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let mut num: Vec<i32> = vec![0; matrix.len()];
    (0..matrix.len()).for_each(|r| {
      let mut sum: i32 = 0;
      (0..matrix[0].len()).for_each(|c| {
        sum += matrix[r][c] << c;
      });
      num[r] = sum;
    });

    let mut max: i32 = 0;
    let mut idx: i32 = (1 << num_select) - 1;
    while idx < 1 << matrix[0].len() {
      let mut col_count: i32 = 0;
      (0..matrix.len()).for_each(|r| {
        if num[r] & idx == num[r] {
          col_count += 1;
        }
      });
      max = std::cmp::max(max, col_count);

      let c = idx & -idx;
      let r = idx + c;
      idx = ((r ^ idx) >> 2) / c | r;
    }
    max
  }

  pub fn maximum_rows2(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let mut num: Vec<i32> = vec![0; matrix.len()];
    (0..matrix.len()).for_each(|r| {
      let mut sum: i32 = 0;
      (0..matrix[0].len()).for_each(|c| {
        sum += matrix[r][c] << c;
      });
      num[r] = sum;
    });

    let mut max: i32 = 0;
    let mut idx: i32 = 1;
    while idx < 1 << matrix[0].len() {
      let mut count: i32 = 0;
      (0..matrix[0].len()).for_each(|r| {
        if idx & 1 << r != 0 {
          count += 1;
        }
      });
      if count == num_select {
        let mut col_count: i32 = 0;
        (0..matrix.len()).for_each(|r| {
          if num[r] & idx == num[r] {
            col_count += 1;
          }
        });
        max = std::cmp::max(max, col_count);
      }

      idx += 1;
    }
    max
  }

  pub fn maximum_rows3(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let mut matrix = matrix;
    fn dfs(
      matrix: &mut Vec<Vec<i32>>,
      num_select: i32,
      select_count: i32,
      col: usize,
      max: &mut i32,
    ) {
      if col == matrix[0].len() || select_count == num_select {
        let mut row_count: i32 = 0;
        (0..matrix.len()).for_each(|r| {
          let mut sum: i32 = 0;
          (0..matrix[0].len()).for_each(|c| {
            sum += matrix[r][c];
          });
          if sum == 0 {
            row_count += 1;
          }
        });
        *max = std::cmp::max(*max, row_count);
        return;
      }

      let mut visit: Vec<usize> = vec![];
      (0..matrix.len()).for_each(|r| {
        if matrix[r][col] == 1 {
          visit.push(r);
          matrix[r][col] = 0;
        }
      });

      dfs(matrix, num_select, select_count + 1, col + 1, max);
      visit.iter().for_each(|&r| {
        matrix[r][col] = 1;
      });
      dfs(matrix, num_select, select_count, col + 1, max);
    }

    let mut max: i32 = 0;
    dfs(&mut matrix, num_select, 0, 0, &mut max);
    max
  }
}
