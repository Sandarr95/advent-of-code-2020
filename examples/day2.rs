use aocf::Aoc;
use regex::Regex;

fn to_usize(x: &str) -> Option::<usize> {
  x.parse::<usize>().ok()
}
fn validate_policy1(re: &regex::Regex, line: &str) -> bool {
  if line == "" {
    return false;
  }
  let obj = re.captures(line).unwrap();
  let low = to_usize(obj.name("low").map_or("", |m| m.as_str())).unwrap();
  let high = to_usize(obj.name("high").map_or("", |m| m.as_str())).unwrap();
  let letter = obj.name("letter").map_or("", |m| m.as_str());
  let password = obj.name("password").map_or("", |m| m.as_str());
  let letter_count = password.split("").filter(|x| x == &letter).count();
  return low <= letter_count && letter_count <= high;
}

fn validate_policy2(re: &regex::Regex, line: &str) -> bool {
  if line == "" {
    return false;
  }
  let obj = re.captures(line).unwrap();
  let low = to_usize(obj.name("low").map_or("", |m| m.as_str())).unwrap();
  let high = to_usize(obj.name("high").map_or("", |m| m.as_str())).unwrap();
  let letter = obj.name("letter").map_or("", |m| m.as_str());
  let password = obj.name("password").map_or("", |m| m.as_str());
  let pass_in_vec: Vec<&str> = password.split("").collect();
  // split("") will start every vec with "", thus we can use it as if it was one-indexed, not zero-indexed.
  (pass_in_vec[low] == letter) != (pass_in_vec[high] == letter)
}

fn main() {
  let re = Regex::new(r"(?P<low>\d+)-(?P<high>\d+) (?P<letter>.): (?P<password>.*)").unwrap();

  let mut aoc = Aoc::new()
              .year(Some(2020))
              .day(Some(2))
              .init()
              .unwrap();

  if let Ok(input) = aoc.get_input(false) {
    let lines = input.split("\n");
    let count_policy1 = lines.clone().filter(|line| validate_policy1(&re, &line)).count();
    let count_policy2 = lines.clone().filter(|&line| validate_policy2(&re, &line)).count();
    println!("Part 1: {}", count_policy1);
    println!("Part 2: {}", count_policy2);
  }
}
