use std::collections::{HashSet, VecDeque};

use crate::util::grid::{Cell, Grid, GridCoords};

const PEAK_VALUE: u32 = 9;

fn score_trailhead(cell: Cell<'_, u32>) -> usize {
    let grid = cell.grid;
    let mut queue = VecDeque::from_iter(std::iter::once(cell.data.position));
    let mut seen: HashSet<_> = HashSet::from_iter(std::iter::once(cell.data.to_owned()));
    while let Some(position) = queue.pop_back() {
        let current = grid.get(position).unwrap();
        for adjacent_data in current
            .adjacent4()
            .filter(|other| *other.data.value == current.data.value + 1)
            .map(|other| other.data.to_owned())
        {
            if seen.insert(adjacent_data) {
                queue.push_front(adjacent_data.position);
            }
        }
    }
    seen.into_iter().filter(|data| data.value == PEAK_VALUE).count()
}

fn rate_trailhead(cell: Cell<'_, u32>) -> usize {
    fn dfs(cell: Cell<'_, u32>, stack: &mut Vec<GridCoords>) -> usize {
        if *cell.data.value == PEAK_VALUE {
            return 1;
        }
        stack.push(cell.data.position);
        let mut rating = 0;
        let adjacents: Vec<_> = cell
            .adjacent4()
            .filter(|other| !stack.contains(&other.data.position) && *other.data.value == cell.data.value + 1)
            .collect();
        for adjacent in adjacents {
            rating += dfs(adjacent, stack);
        }
        stack.pop();
        rating
    }
    let mut stack = Vec::new();
    dfs(cell, &mut stack)
}

fn solve(input: crate::Input, f: impl Fn(Cell<'_, u32>) -> usize) -> usize {
    Grid::<u32>::parse(input).find_all(|&h| h == 0).fold(0, |total, head| total + f(head))
}

pub fn part1(input: crate::Input) -> usize {
    solve(input, score_trailhead)
}

pub fn part2(input: crate::Input) -> usize {
    solve(input, rate_trailhead)
}
