use aocf::Aoc;

fn tree_at(skip_x: usize, skip_y: usize, line_nr: usize, line: &str) -> bool {
  if line_nr % skip_y != 0 {
    false
  } else {
    let idx = (skip_x * (line_nr / skip_y)) % line.len();
    line.chars().nth(idx) == Some('#')
  }
}

fn main() {
  let mut aoc = Aoc::new()
              .year(Some(2020))
              .day(Some(3))
              .init()
              .unwrap();

  let input = aoc.get_input(false);

  if let Ok(i) = input {
    let lines_iter = i.split('\n');
    let mut line_number = 0;
    let mut trees_1_1: u64 = 0;
    let mut trees_3_1: u64 = 0;
    let mut trees_5_1: u64 = 0;
    let mut trees_7_1: u64 = 0;
    let mut trees_1_2: u64 = 0;
    for hori_line in lines_iter {
      if hori_line.len() == 0 { continue }
      if tree_at(1, 1, line_number, hori_line) { trees_1_1 += 1; }
      if tree_at(3, 1, line_number, hori_line) { trees_3_1 += 1; }
      if tree_at(5, 1, line_number, hori_line) { trees_5_1 += 1; }
      if tree_at(7, 1, line_number, hori_line) { trees_7_1 += 1; }
      if tree_at(1, 2, line_number, hori_line) { trees_1_2 += 1; }
      line_number += 1;
    }
    println!("Part 1: {}", trees_3_1);
    println!("Part 2: {}", trees_1_1 * trees_3_1 * trees_5_1 * trees_7_1 * trees_1_2);
  }
}
