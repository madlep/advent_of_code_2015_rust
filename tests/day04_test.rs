use aoc2015::days::day04::{part1, part2};

// these are slow. ignore by default
#[ignore]
#[test]
fn part1_example_data() {
    assert_eq!(part1("abcdef"), "609043");
    assert_eq!(part1("pqrstuv"), "1048970");
}

#[ignore]
#[test]
fn part2_example_data() {
    assert_eq!(part2("abcdef"), "6742839");
    assert_eq!(part2("pqrstuv"), "5714438");
}
