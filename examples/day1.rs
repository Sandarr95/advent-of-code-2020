use aocf::Aoc;
use std::collections::HashSet;
use std::iter::FromIterator;

fn to_u32(x: &str) -> Option::<u32> {
  x.parse::<u32>().ok()
}

fn main() {
  let mut aoc = Aoc::new()
              .year(Some(2020))
              .day(Some(1))
              .init()
              .unwrap();

  let input = aoc.get_input(false);

  if let Ok(i) = input {
    let ints_iter = i.split("\n").filter_map(to_u32);
    let set_of_ints = HashSet::<u32>::from_iter(ints_iter.clone());
    for part_of_2020 in ints_iter.clone() {
      let guess_of_2020 = 2020 - part_of_2020;
      if set_of_ints.contains(&guess_of_2020) {
        println!("Part 1: {}", guess_of_2020 * part_of_2020);
        break;
      }
    }

    let ints_vec = Vec::from_iter(ints_iter);
    for part1_of_2020 in &ints_vec {
      for part2_of_2020 in &ints_vec {
        for part3_of_2020 in &ints_vec {
          //print!("{}, ", part1_of_2020 + part2_of_2020 + part3_of_2020);
          if part1_of_2020 + part2_of_2020 + part3_of_2020 == 2020 {
            println!("Part 2: {}", part1_of_2020 * part2_of_2020 * part3_of_2020);
            return;
          }
        }
      }
    }
  }
}
