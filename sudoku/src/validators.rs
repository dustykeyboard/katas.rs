/// Validate if a number can be placed in a position on a given grid
pub fn is_valid(grid: [[usize; 9]; 9], y: usize, x: usize, n: usize) -> bool {
  // Fail validation if n exists in row
  for i in 0..9 {
    if grid[y][i] == n {
      return false;
    }
  }

  // Fail validation if n exists in column
  for i in 0..9 {
    if grid[i][x] == n {
      return false;
    }
  }

  // Fail validation if n exists in square
  let x0 = (x/3)*3;
  let y0 = (y/3)*3;
  for i in 0..3 {
    for j in 0..3 {
      if grid[i+y0][j+x0] == n {
        return false
      }
    }
  }

  // Validation passes
  return true;
}

#[cfg(test)]
mod tests {
  use super::*;
  
  static GRID: [[usize; 9]; 9] = [
    [1,0,0,0,0,0,0,0,0],
    [0,2,0,0,0,0,0,0,0],
    [0,0,3,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,6,0,0,0],
    [0,0,0,0,0,0,7,0,0],
    [0,0,0,0,0,0,0,8,9],
  ];

  #[test]
  fn row_is_not_valid() {
    assert_eq!(is_valid(GRID, 0, 5, 1), false);
  }

  #[test]
  fn column_is_not_valid() {
    assert_eq!(is_valid(GRID, 5, 0, 1), false);
  }

  #[test]
  fn house_is_not_valid() {
    assert_eq!(is_valid(GRID, 2, 1, 1), false);
  }

  #[test]
  fn grid_is_valid() {
    assert_eq!(is_valid(GRID, 1, 4, 1), true);
  }
}