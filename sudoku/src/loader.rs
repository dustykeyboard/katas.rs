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

pub fn deserialize(source: &str) -> [[usize; 9]; 9] {
  let mut grid: [[usize; 9]; 9] = [[0; 9]; 9];
  let chars: Vec<char> = source.chars().collect();
  for i in 0..9 {
    for j in 0..9 {
      grid[i][j] = 0;
      let value = chars[i * 10 + j];
      if value.is_numeric() {
        grid[i][j] = value.to_string().parse::<usize>().unwrap();
      }
    }
  }
  return grid;
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
  static STRING: &str = "\
100000000
020000000
003000000
000000000
000000000
000000000
000006000
000000700
000000089
";

  #[test]
  fn grid_to_string() {
    assert_eq!(serialize(GRID), STRING);
  }

  #[test]
  fn string_to_grid() {
    assert_eq!(deserialize(STRING), GRID);
  }
}