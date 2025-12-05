pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> i32 {
    0
}

fn solve2(input: &String) -> i32 {
    0
}

#[test]
fn part_one() {
    let data = String::from(
        "\
<test_input_here>
",
    );

    let res = solve1(&data);
    assert_eq!(0, res)
}


#[test]
fn part_two() {
    let data = String::from(
        "\
<test_input_here>
",
    );

    let res = solve2(&data);
    assert_eq!(0, res)
}
