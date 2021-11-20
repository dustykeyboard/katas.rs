use rand::seq::SliceRandom;
use rand::thread_rng;

// generate filled grid
pub fn generate_grid() -> [[usize; 9]; 9] {
    let mut grid: [[usize; 9]; 9] = [[0; 9]; 9];
    let seed: [usize; 9] = generate_seed_column();

    grid[0] = [seed[0],seed[1],seed[2],seed[3],seed[4],seed[5],seed[6],seed[7],seed[8]];
    grid[1] = [seed[3],seed[4],seed[5],seed[6],seed[7],seed[8],seed[0],seed[1],seed[2]];
    grid[2] = [seed[6],seed[7],seed[8],seed[0],seed[1],seed[2],seed[3],seed[4],seed[5]];
    grid[4] = [seed[1],seed[2],seed[3],seed[4],seed[5],seed[6],seed[7],seed[8],seed[0]];
    grid[4] = [seed[4],seed[5],seed[6],seed[7],seed[8],seed[0],seed[1],seed[2],seed[3]];
    grid[5] = [seed[7],seed[8],seed[0],seed[1],seed[2],seed[3],seed[4],seed[5],seed[6]];
    grid[6] = [seed[2],seed[3],seed[4],seed[5],seed[6],seed[7],seed[8],seed[0],seed[1]];
    grid[7] = [seed[5],seed[6],seed[7],seed[8],seed[0],seed[1],seed[2],seed[3],seed[4]];
    grid[8] = [seed[8],seed[0],seed[1],seed[2],seed[3],seed[4],seed[5],seed[6],seed[7]];

    return grid;
}

// generate column
fn generate_seed_column() -> [usize; 9] {
  let mut rng = thread_rng();
  let mut column: [usize; 9] = [1,2,3,4,5,6,7,8,9];
  column.shuffle(&mut rng);
  return column;
}

// remove numbers
pub fn remove_numbers(grid: &mut [[usize; 9]; 9]) {
  let mut rng = thread_rng();
  let mut order: [usize; 9] = [0,1,2,3,4,5,6,7,8];
  order.shuffle(&mut rng);

  for i in order {
    for j in order {
      let possibilities = num_possibilities(*grid, i, j);
      if possibilities <= 1 {
        grid[i][j] = 0;
      }
    }
  }
}

// what numbers are possible at this position
fn num_possibilities(grid: [[usize; 9]; 9], y: usize, x: usize) -> u32 {
  let mut count = 0;

  for n in 1..9 {
    let mut duplicate: bool = false;
    // Fail validation if n exists in row
    for i in 0..9 {
      if i != x && grid[y][i] == n {
        duplicate = true;
        break;
      }
    }

    // Fail validation if n exists in column
    for i in 0..9 {
      if i != y && grid[i][x] == n {
        duplicate = true;
        break;
      }
    }

    // Fail validation if n exists in square
    let x0 = (x/3)*3;
    let y0 = (y/3)*3;
    for i in 0..3 {
      for j in 0..3 {
        if i != y && j != x && grid[i+y0][j+x0] == n {
          duplicate = true;
          break;
        }
      }
    }

    // Validation passes
    if !duplicate {
      count += 1;
    };
  }

  return count;
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
  fn counts_num_possibilities() {
    assert_eq!(num_possibilities(GRID, 0, 5), 0);
  }
}