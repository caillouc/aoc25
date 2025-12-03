pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> u32 {
    let mut sum = 0;
    input.lines().for_each(|l| {
        let mut max_char = 0;
        let mut second_max_char = 0;
        if !l.is_empty() {
            l[..l.len() - 1].chars().for_each(|c| {
                let c_int = c.to_digit(10).unwrap();
                if c_int > max_char {
                    max_char = c_int;
                    second_max_char = 0;
                } else if c_int > second_max_char {
                    second_max_char = c_int;
                }
            });
        }
        if let Some(last_char) = l.chars().last()
            && let Some(c_int) = last_char.to_digit(10)
            && c_int > second_max_char
        {
            second_max_char = c_int;
        }
        sum += 10 * max_char + second_max_char;
    });
    sum
}

fn solve2(input: &String) -> u64 {
    let mut sum: u64 = 0;
    input.lines().for_each(|l| {
        let mut maxes: Vec<u32> = vec![0; 12];
        if !l.is_empty() {
            l.chars().enumerate().for_each(|(c_index, c)| {
                let c_int = c.to_digit(10).unwrap();
                let mut max_found = false;
                maxes.iter_mut().enumerate().for_each(|(max_index, max)| {
                    if c_int > *max && c_index <= l.len() - 12 + max_index && !max_found {
                        *max = c_int;
                        max_found = true;
                    } else if max_found {
                        *max = 0;
                    }
                });
            });
        }
        sum += maxes.iter().enumerate().fold(0, |acc: u64, (index, max)| {
            acc + u64::pow(10, (11 - index) as u32) * *max as u64
        });
    });
    sum
}

// fn solve2(input: &String) -> i32 {0}

#[test]
fn part_one() {
    let data = String::from(
        "\
987654321111111
811111111111119
234234234234278
818181911112111
",
    );

    let res = solve1(&data);
    assert_eq!(357, res)
}

#[test]
fn part_two() {
    let data = String::from(
        "\
987654321111111
811111111111119
234234234234278
818181911112111
",
    );

    let res = solve2(&data);
    assert_eq!(3121910778619, res)
}
