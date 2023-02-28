use aoc2015::days::day01::{part1, part2};

#[test]
fn part1_example_data() {
    assert_eq!(part1("(())"), "0");
    assert_eq!(part1("()()"), "0");
    assert_eq!(part1("((("), "3");
    assert_eq!(part1("(()(()("), "3");
    assert_eq!(part1("))((((("), "3");
    assert_eq!(part1("())"), "-1");
    assert_eq!(part1("))("), "-1");
    assert_eq!(part1(")))"), "-3");
    assert_eq!(part1(")())())"), "-3");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(")"), "1");
    assert_eq!(part2("()())"), "5");
}
