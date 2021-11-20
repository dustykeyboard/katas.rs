use std::fs;
use std::io::Write;

mod loader;
mod validators;
mod generators;

fn main() {
  let mut grid: [[usize; 9]; 9];

  // Read input from "puzzle.txt" file
  let source = fs::read_to_string("puzzle.txt")
    .unwrap_or("".to_string());
  
  if source.is_empty() {
    println!("No puzzle file found, generating...");
    grid = generators::generate_grid();
    generators::remove_numbers(&mut grid);

    // Write puzzle to disk
    let input = loader::serialize(&grid);
    // println!("{}", input);

    let mut file = std::fs::File::create("puzzle.txt").expect("create failed");
    file.write_all(input.as_bytes()).expect("write failed");
    println!("puzzle written to puzzle.txt" );
    println!("{}", input);
  } else {
    println!("Parsing puzzle.txt and solving...");
    // Parse grid from string
    grid = loader::deserialize(&source);
  
    // Recursively search for valid solutions
    validators::solve(&mut grid);
    
    // Write solution to disk
    let output = loader::serialize(&grid);
    // println!("{}", output);
    
    let mut file = std::fs::File::create("solution.txt").expect("create failed");
    file.write_all(output.as_bytes()).expect("write failed");
    println!("solution written to solution.txt" );
    println!("{}", output);
  }
}
