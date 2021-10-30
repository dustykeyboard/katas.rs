use std::fs;
use std::io::Write;

mod loader;
mod validators;

fn solve(&(mut grid): &[[usize; 9]; 9]) -> bool {
  let mut filled = true;
  for y in 0..9 {
    for x in 0..9 {
      if grid[y][x] == 0 {
        filled = false;
        for n in 1..=9 {
          if validators::is_valid(grid, y, x, n) {
            grid[y][x] = n;
            if !solve(&grid) {
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
  if filled {
    println!("*** SOLVED! ***");
    let output = loader::serialize(grid);
    println!("{}", output);
  
    // Write solution to disk
    let mut file = std::fs::File::create("solution.txt").expect("create failed");
    file.write_all(output.as_bytes()).expect("write failed");
    println!("solution written to solution.txt" );
    return true;
  } else {
    println!("*** NOT SOLVED! ***");
    return false;
  }
}

fn main() {
  println!("Hello, sudoku!");

  // Read puzzle from "puzzle.txt" file
  
  let puzzle = fs::read_to_string("puzzle.txt")
    .expect("Failed to read puzzle.\nPlease enter your sudoku grid at puzzle.txt");

  // Parse grid from string
  let grid: [[usize; 9]; 9] = loader::deserialize(&puzzle);

  // Recursively search for valid solutions
  solve(&grid);
}
