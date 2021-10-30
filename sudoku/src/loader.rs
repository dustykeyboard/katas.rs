pub fn serialize(grid: [[usize; 9]; 9]) -> String {
  let mut output = String::new();
  for i in 0..9 {
    for j in 0..9 {
      output.push_str(grid[i][j].to_string().as_str());
    }
    output.push('\n');
  }
  return output;
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
  static STRING: &str = "\
100000000
000000000
000000000
000000000
000000000
000000000
000000000
000000000
000000000
";

  #[test]
  fn grid_to_string() {
    assert_eq!(serialize(GRID), STRING);
  }

  // #[test]
  // fn string_to_grid() {
  //   assert_eq!(deserialize(STRING), GRID);
  // }
}