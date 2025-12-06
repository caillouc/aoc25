use crate::utils::parser;

pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> u64 {
    let splitted: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|value| if value.is_empty() { None } else { Some(value) })
                .collect()
        })
        .collect();
    let signes = splitted.last().unwrap();
    let mut values: Vec<Vec<u64>> = vec![vec![]; splitted[0].len()];
    splitted[..splitted.len() - 1].iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(index, it)| values[index].push(it.parse().unwrap()));
    });
    values.iter().zip(signes).fold(0, |acc, (col, signe)| {
        acc + col
            .iter()
            .fold(if *signe == "*" { 1 } else { 0 }, |inside_acc, it| {
                if *signe == "*" {
                    inside_acc * it
                } else {
                    inside_acc + it
                }
            })
    })
}

fn solve2(input: &String) -> u64 {
    let signes: Vec<&str> = input.lines().last().unwrap().split(' ').filter(|it| !it.is_empty()).collect();
    let chars = parser::parse_double_vec(input)[..input.lines().count() - 1].to_vec();
    let mut columns = vec![vec![]; chars[0].len()];
    chars.iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(index, c)| columns[index].push(*c));
    });
    let mut values: Vec<Vec<u64>> = vec![vec![]; signes.len()];
    let mut current_index = 0;
    for col in columns {
        let val_str: String = col.iter().collect::<String>();
        if val_str.trim().is_empty() {
            current_index += 1;
            continue;
        } else {
            values[current_index].push(val_str.trim().parse().unwrap());
        }
    }
    values.iter().zip(signes).fold(0, |acc, (col, signe)| {
        acc + col
            .iter()
            .fold(if signe == "*" { 1 } else { 0 }, |inside_acc, it| {
                if signe == "*" {
                    inside_acc * it
                } else {
                    inside_acc + it
                }
            })
    })
}

#[test]
fn part_one() {
    let data = String::from(
        "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
",
    );

    let res = solve1(&data);
    assert_eq!(4277556, res)
}

#[test]
fn part_two() {
    let data = String::from(
        "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
",
    );

    let res = solve2(&data);
    assert_eq!(3263827, res)
}
