pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> i32 {
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut better_ranges = vec![];
    let mut is_range = true;
    let mut valid_ingredient = 0;
    for l in input.lines() {
        if l.is_empty() && is_range {
            let mut current_max = 0;
            ranges.iter().for_each(|range| {
                if better_ranges.is_empty() || range.0 > current_max {
                    better_ranges.push(*range);
                    current_max = range.1;
                } else if range.1 > current_max {
                    *better_ranges.last_mut().unwrap() = (better_ranges.last().unwrap().0, range.1);
                    current_max = range.1;
                }
            });
            is_range = false;
        }
        if is_range {
            let min = l.split('-').next().unwrap().parse().unwrap();
            let max = l.split('-').last().unwrap().parse().unwrap();
            ranges.push((min, max));
            ranges.sort_by(|a, b| {
                if a.0 == b.1 {
                    a.1.cmp(&b.1)
                } else {
                    a.0.cmp(&b.0)
                }
            });
        } else {
            if !l.is_empty() {
                let ingredient: u64 = l.parse().unwrap();
                for range in ranges.iter() {
                    if ingredient < range.0 {
                        break;
                    } else if ingredient <= range.1 {
                        valid_ingredient += 1;
                        break;
                    }
                }
            }
        }
    }
    valid_ingredient
}

fn solve2(input: &String) -> u64 {
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut better_ranges = vec![];
    for l in input.lines() {
        if l.is_empty() {
            let mut current_max = 0;
            ranges.iter().for_each(|range| {
                if better_ranges.is_empty() || range.0 > current_max {
                    better_ranges.push(*range);
                    current_max = range.1;
                } else if range.1 > current_max {
                    *better_ranges.last_mut().unwrap() = (better_ranges.last().unwrap().0, range.1);
                    current_max = range.1;
                }
            });
            break;
        }
        let min = l.split('-').next().unwrap().parse().unwrap();
        let max = l.split('-').last().unwrap().parse().unwrap();
        ranges.push((min, max));
        ranges.sort_by(|a, b| {
            if a.0 == b.1 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
    }
    better_ranges.iter().fold(0, |acc, range| acc + (range.1 - range.0) + 1)
}

#[test]
fn part_one() {
    let data = String::from(
        "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
",
    );

    let res = solve1(&data);
    assert_eq!(3, res)
}

#[test]
fn part_two() {
    let data = String::from(
        "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
",
    );

    let res = solve2(&data);
    assert_eq!(14, res)
}
