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

pub fn solve(grid: &mut [[usize; 9]; 9]) -> bool {
  let mut filled = true;
  for y in 0..9 {
    for x in 0..9 {
      if grid[y][x] == 0 {
        filled = false;
        for n in 1..=9 {
          if is_valid(*grid, y, x, n) {
            grid[y][x] = n;

            if !solve(grid) {
              grid[y][x] = 0;
            } else {
              return true;
            }
          }
        }
        return filled;
      }
    }
  }

  return filled;
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

  #[test]
  fn solves() {
    let mut grid: [[usize; 9]; 9] = [
      [6,0,0,0,3,0,0,0,9],
      [0,1,0,0,0,0,0,8,0],
      [0,0,5,9,0,6,3,0,0],
      [0,0,6,3,0,4,8,0,0],
      [1,0,0,0,6,0,0,0,3],
      [0,0,2,1,0,9,4,0,0],
      [0,0,7,5,0,3,1,0,0],
      [0,9,0,0,0,0,0,4,0],
      [2,0,0,0,1,0,0,0,6],
    ];
    // 600|030|009
    // 010|000|080
    // 005|906|300
    // ---+---+---
    // 006|304|800
    // 100|060|003
    // 002|109|400
    // ---+---+---
    // 007|503|100
    // 090|000|040
    // 200|010|006
    let solution: [[usize; 9]; 9] = [
      [6,2,4,8,3,1,5,7,9],
      [9,1,3,2,7,5,6,8,4],
      [7,8,5,9,4,6,3,1,2],
      [5,7,6,3,2,4,8,9,1],
      [1,4,9,7,6,8,2,5,3],
      [8,3,2,1,5,9,4,6,7],
      [4,6,7,5,9,3,1,2,8],
      [3,9,1,6,8,2,7,4,5],
      [2,5,8,4,1,7,9,3,6],
    ];
    // 624|831|579
    // 913|275|684
    // 785|946|312
    // ---+---+---
    // 576|324|891
    // 149|768|253
    // 832|159|467
    // ---+---+---
    // 467|593|128
    // 391|682|745
    // 258|417|936

    solve(&mut grid);

    assert_eq!(grid, solution)
  }
}