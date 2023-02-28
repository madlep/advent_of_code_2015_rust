use aoc2015::days::day02::{part1, part2};

#[test]
fn part1_example_data() {
    assert_eq!(part1("2x3x4"), "58");
    assert_eq!(part1("1x1x10"), "43");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2("2x3x4"), "34");
    assert_eq!(part2("1x1x10"), "14");
}
