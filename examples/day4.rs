use aocf::Aoc;
use regex::Regex;

fn main() {
  let re = Regex::new(r"^(cid:[\S]*\s)?((byr|iyr|eyr|hgt|hcl|ecl|pid):[\S]*\s?(cid:[\S]*\s?)?){7}$").unwrap();
  let strict_re = Regex::new(r"^(cid:[\S]*\s)?((byr:(19[2-9][0-9]|200[0-2])|iyr:20(1[0-9]|20)|eyr:20(2[0-9]|30)|hgt:(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)|hcl:#[0-9a-f]{6}|ecl:(amb|blu|brn|gry|grn|hzl|oth)|pid:[0-9]{9})\s?(cid:[\S]*\s?)?){7}$").unwrap();

  let mut aoc = Aoc::new()
              .year(Some(2020))
              .day(Some(4))
              .init()
              .unwrap();

  if let Ok(input) = aoc.get_input(false) {
    let lines = input.split("\n\n");
    let count_policy1 = lines.clone().filter(|&line| re.is_match(line)).count();
    let count_policy2 = lines.clone().filter(|&line| strict_re.is_match(line)).count();
    println!("Part 1: {}", count_policy1);
    println!("Part 2: {}", count_policy2);
  }
}
