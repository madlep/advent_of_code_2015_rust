use aoc2015::days::day08::{part1, part2};

#[test]
fn part1_example_data() {
    let data = r#"""
"abc"
"aaa\\aaa"
"\x27""#;
    assert_eq!(part1(data), "12");
}

// #[test]
// fn part2_example_data() {
//     assert_eq!(part2("abcdef"), "6742839");
//     assert_eq!(part2("pqrstuv"), "5714438");
// }
