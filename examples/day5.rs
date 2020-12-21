use aocf::Aoc;
use std::collections::HashSet;
use std::iter::FromIterator;

fn check_num(ids: &HashSet::<isize>, num: isize) -> bool {
  !ids.contains(&num) && ids.contains(&(num+1)) && ids.contains(&(num-1))
}

fn main() {
  let mut aoc = Aoc::new()
              .year(Some(2020))
              .day(Some(5))
              .init()
              .unwrap();

  let input = aoc.get_input(false);

  if let Ok(i) = input {
    let lines_iter = i.split('\n');
    let ids_iter = lines_iter
      .map(|line| line.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1"))
      .filter_map(|line| isize::from_str_radix(&line, 2).ok());
    let ids = HashSet::<isize>::from_iter(ids_iter.clone());
    let max_num = ids_iter.fold(0, |max, item| if max > item { max } else { item });

    let mut cnt = 0;
    let my_num = loop { if check_num(&ids, cnt) { break cnt } cnt+=1; };

    println!("Part 1: {}", max_num);
    println!("Part 2: {}", my_num);
  }
}
