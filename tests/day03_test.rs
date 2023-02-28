use aoc2015::days::day03::{part1, part2};

#[test]
fn part1_example_data() {
    assert_eq!(part1(">"), "2");
    assert_eq!(part1("^>v<"), "4");
    assert_eq!(part1("^v^v^v^v^v"), "2");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2("^v"), "3");
    assert_eq!(part2("^>v<"), "3");
    assert_eq!(part2("^v^v^v^v^v"), "11");
}
