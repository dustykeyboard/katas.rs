use std::fs;
use std::io::Write;

mod loader;
mod validators;

fn main() {
  // Read puzzle from "puzzle.txt" file
  let puzzle = fs::read_to_string("puzzle.txt")
    .expect("Failed to read puzzle.\nPlease enter your sudoku grid at puzzle.txt");

  // Parse grid from string
  let mut grid: [[usize; 9]; 9] = loader::deserialize(&puzzle);

  // Recursively search for valid solutions
  validators::solve(&mut grid);

  let output = loader::serialize(&grid);
  println!("{}", output);

  // Write solution to disk
  let mut file = std::fs::File::create("solution.txt").expect("create failed");
  file.write_all(output.as_bytes()).expect("write failed");
  println!("solution written to solution.txt" );
}
