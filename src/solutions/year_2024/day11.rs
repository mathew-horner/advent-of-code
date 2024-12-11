use std::collections::HashMap;

fn parse(input: crate::Input) -> Vec<u64> {
    input.read_all().split(' ').map(|token| token.trim().parse().unwrap()).collect()
}

fn solve(input: crate::Input, blinks: usize) -> usize {
    let mut cache = HashMap::new();
    parse(input).into_iter().map(|stone| solve_inner(&mut cache, stone, blinks)).sum()
}

type Stone = u64;

/// Map of (Stone, Blinks Left) -> The number of stones after execution.
type Cache = HashMap<(Stone, usize), usize>;

fn solve_inner(cache: &mut Cache, stone: Stone, blinks: usize) -> usize {
    if let Some(result) = cache.get(&(stone, blinks)) {
        return *result;
    }
    if blinks == 0 {
        return 1;
    }
    let result = match stone {
        0 => solve_inner(cache, 1, blinks - 1),
        other => {
            let digits = other.ilog(10) + 1;
            if digits % 2 == 0 {
                let m = 10_u64.pow(digits / 2);
                let left = other / m;
                let right = other % m;
                solve_inner(cache, left, blinks - 1) + solve_inner(cache, right, blinks - 1)
            } else {
                solve_inner(cache, other * 2024, blinks - 1)
            }
        },
    };
    cache.insert((stone, blinks), result);
    result
}

pub fn part1(input: crate::Input) -> usize {
    solve(input, 25)
}

pub fn part2(input: crate::Input) -> usize {
    solve(input, 75)
}
