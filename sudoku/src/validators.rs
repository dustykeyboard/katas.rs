/// Validate if a number can be placed in a position on a given grid
pub fn duplicate(grid: [[usize; 9]; 9], x: usize, y: usize, n: usize) -> bool {
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
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
  ];

  #[test]
  fn duplicate_row() {
    assert_eq!(duplicate(GRID, 0, 5, 1), false);
  }

  #[test]
  fn duplicate_column() {
    assert_eq!(duplicate(GRID, 5, 0, 1), false);
  }

  #[test]
  fn duplicate_square() {
    assert_eq!(duplicate(GRID, 2, 2, 1), false);
  }

  #[test]
  fn not_duplicate() {
    assert_eq!(duplicate(GRID, 0, 0, 2), true);
  }
}