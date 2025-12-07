use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::parser;
use crate::utils::position::Position;

pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> i32 {
    let grid = parser::parse_grid(input);
    let s_pos = Position::from(
        grid.first_row()
            .unwrap()
            .iter()
            .position(|it| *it == 'S')
            .unwrap(),
        0,
    );
    let mut current_beams = HashSet::new();
    current_beams.insert(s_pos.x());
    let mut nb_split = 0;
    for (row_index, row) in grid.rows().enumerate() {
        let mut new_beams = HashSet::new();
        if row_index == 0 {
            continue;
        }
        for xs in current_beams.iter() {
            if row[*xs] == '^' {
                nb_split += 1;
                if grid.contains_pos(Position::from(xs + 1, row_index)) {
                    new_beams.insert(xs + 1);
                }
                if grid.contains_pos(Position::from(xs - 1, row_index)) {
                    new_beams.insert(xs - 1);
                }
            } else {
                new_beams.insert(*xs);
            }
        }
        current_beams = new_beams;
    }
    nb_split
}

fn solve2(input: &String) -> i64 {
    let grid = parser::parse_grid(input);
    let s_pos = Position::from(
        grid.first_row()
            .unwrap()
            .iter()
            .position(|it| *it == 'S')
            .unwrap(),
        0,
    );
    let mut current_beams = HashMap::new();
    current_beams.insert(s_pos.x(), 1);
    let mut nb_path = 1;
    for (row_index, row) in grid.rows().enumerate() {
        let mut new_beams = HashMap::new();
        if row_index == 0 {
            continue;
        }
        for (xs, count) in current_beams.iter() {
            if row[*xs] == '^' {
                nb_path += count;
                if grid.contains_pos(Position::from(xs + 1, row_index)) {
                    *new_beams.entry(xs+1).or_insert(0) += count;
                }
                if grid.contains_pos(Position::from(xs - 1, row_index)) {
                    *new_beams.entry(xs-1).or_insert(0) += count;
                }
            } else {
                *new_beams.entry(*xs).or_insert(0) += count;
            }
        }
        current_beams = new_beams;
    }
    nb_path
}

#[test]
fn part_one() {
    let data = String::from(
        "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
    );

    let res = solve1(&data);
    assert_eq!(21, res)
}

#[test]
fn part_two() {
    let data = String::from(
        "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
    );

    let res = solve2(&data);
    assert_eq!(40, res)
}
