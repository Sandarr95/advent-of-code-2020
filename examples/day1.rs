use aocf::Aoc;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
  let mut aoc = Aoc::new()
              .year(Some(2020))
              .day(Some(1))
              .init()
              .unwrap();

  let input = aoc.get_input(false);

  if let Ok(i) = input {
    let ints_iter = i.split("\n").filter(|&x| x != "").map(|x| x.parse::<u32>().unwrap());
    let set_of_ints = HashSet::<u32>::from_iter(ints_iter.clone());
    for part_of_2020 in ints_iter {
      let guess_of_2020 = 2020 - part_of_2020;
      if set_of_ints.contains(&guess_of_2020) {
        return println!("{}", guess_of_2020 * part_of_2020);
      }
    }
  }

}
