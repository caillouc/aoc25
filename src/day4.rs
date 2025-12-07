use crate::utils::parser;
use crate::utils::position::Position;

pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> i32 {
    let mut count = 0;
    let grid = parser::parse_double_vec(input);
    grid.iter().enumerate().for_each(|(y, line)|{
        line.iter().enumerate().for_each(|(x, c)| {
            if *c == '@' {
                let mut local_count = 0;
                let current_pos = Position::from(x, y);
                for adj in current_pos.adjacent_pos(line.len(), grid.len()) {
                    if let Some(pos) = adj && grid[pos.y()][pos.x()] == '@' {
                        local_count += 1;
                    }
                }
                if local_count < 4 {
                    count += 1
                }
            }
        });
    });
    count
}

fn solve2(input: &String) -> i32 {
    let mut count: i32 = 0;
    let mut to_remove = vec![];
    let mut grid = parser::parse_double_vec(input);
    loop {
        grid.iter().enumerate().for_each(|(y, line)|{
            line.iter().enumerate().for_each(|(x, c)| {
                if *c == '@' {
                    let mut local_count = 0;
                    let current_pos = Position::from(x, y);
                    for adj in current_pos.adjacent_pos(line.len(), grid.len()) {
                        if let Some(pos) = adj && grid[pos.y()][pos.x()] == '@' {
                            local_count += 1;
                        }
                    }
                    if local_count < 4 {
                        to_remove.push(current_pos);
                    }
                }
            });
        });
        if to_remove.is_empty() {break;}
        else {
            count += to_remove.len() as i32;
            to_remove.iter().for_each(|p| grid[p.y()][p.x()] = '.');
            to_remove.clear();
        }
    }
    count
}

#[test]
fn part_one() {
    let data = String::from(
        "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
    );

    let res = solve1(&data);
    assert_eq!(13, res)
}


#[test]
fn part_two() {
    let data = String::from(
        "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
    );

    let res = solve2(&data);
    assert_eq!(43, res)
}
