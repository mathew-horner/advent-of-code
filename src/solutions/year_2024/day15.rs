use crate::util::grid::Grid;
use crate::util::math::{Point2, Vec2};

struct Input {
    grid: Grid<char>,
    moves: Vec<char>,
    robot_pos: Point2<i32>,
}

fn parse(input: crate::Input) -> Input {
    let input = input.read_all();
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut robot_pos = None;
    let grid: Vec<_> = grid
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let objects = line.chars().enumerate();
            let objects = objects.inspect(|&(x, c)| {
                if c == '@' {
                    robot_pos = Some(Point2 { x: x as i32, y: y as i32 });
                }
            });
            objects.map(|(_, c)| c).collect()
        })
        .collect();

    let moves: Vec<_> = moves.chars().filter(|c| !c.is_whitespace()).collect();
    Input { grid, moves, robot_pos: robot_pos.unwrap() }
}

fn score(grid: &Grid<char>, box_repr: char) -> usize {
    grid.into_iter()
        .enumerate()
        .flat_map(|(y, row)| row.into_iter().enumerate().map(move |(x, col)| (x, y, col)))
        .filter(|(_, _, value)| **value == box_repr)
        .fold(0, |total, (x, y, _)| total + (100 * y + x))
}

fn move_offset(move_: char) -> Vec2<i32> {
    match move_ {
        '^' => Vec2 { x: 0, y: -1 },
        'v' => Vec2 { x: 0, y: 1 },
        '<' => Vec2 { x: -1, y: 0 },
        '>' => Vec2 { x: 1, y: 0 },
        _ => panic!("invalid move character: {move_}"),
    }
}

pub fn part1(input: crate::Input) -> usize {
    let Input { mut grid, moves, mut robot_pos } = parse(input);
    moves.into_iter().for_each(|move_| {
        if move_robot1(&mut grid, robot_pos, move_) {
            robot_pos += move_offset(move_);
        }
    });
    score(&grid, 'O')
}

fn move_robot1(grid: &mut Grid<char>, pos: Point2<i32>, move_: char) -> bool {
    let object = grid[pos.y as usize][pos.x as usize];
    match object {
        '#' => return false,
        '@' | 'O' => {},
        '.' => return true,
        _ => unreachable!(),
    };
    let next_coords = pos + move_offset(move_);
    let can_move = move_robot1(grid, next_coords, move_);
    if !can_move {
        return false;
    }
    grid[next_coords.y as usize][next_coords.x as usize] = object;
    grid[pos.y as usize][pos.x as usize] = '.';
    true
}

pub fn part2(input: crate::Input) -> usize {
    let Input { grid, moves, mut robot_pos } = parse(input);
    let mut double_grid: Grid<_> = (0..grid.len())
        .map(|row| {
            (0..grid[0].len())
                .flat_map(|col| match grid[row][col] {
                    'O' => ['[', ']'],
                    '#' => ['#', '#'],
                    '@' => ['@', '.'],
                    '.' => ['.', '.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    robot_pos.x *= 2;
    moves.into_iter().for_each(|move_| {
        if move_robot2(&mut double_grid, robot_pos, move_) {
            robot_pos += move_offset(move_);
        }
    });

    score(&double_grid, '[')
}

fn move_robot2(grid: &mut Grid<char>, pos: Point2<i32>, move_: char) -> bool {
    let is_vertical = matches!(move_, '^' | 'v');
    let additional_offset = match grid[pos.y as usize][pos.x as usize] {
        '#' => return false,
        '[' if is_vertical => Some(1),
        ']' if is_vertical => Some(-1),
        '@' | '[' | ']' => None,
        '.' => return true,
        _ => unreachable!(),
    };
    let offset = move_offset(move_);
    let next_coords: Vec<_> = [Some(pos), additional_offset.map(|x| Point2 { x, y: 0 }).map(|offset| pos + offset)]
        .into_iter()
        .flatten()
        .map(|pos| (pos, pos + offset))
        .collect();
    let mut temp_grid = grid.clone();
    for (_, next) in &next_coords {
        let can_move = move_robot2(&mut temp_grid, *next, move_);
        if !can_move {
            return false;
        }
    }
    *grid = temp_grid;
    for (prev, next) in next_coords {
        let object = grid[prev.y as usize][prev.x as usize];
        grid[next.y as usize][next.x as usize] = object;
        grid[prev.y as usize][prev.x as usize] = '.';
    }
    true
}
