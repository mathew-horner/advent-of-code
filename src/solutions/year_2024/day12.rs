use std::collections::{HashSet, VecDeque};

use crate::util::grid::{Grid, GridCoords};

fn solve(input: crate::Input) -> (usize, usize) {
    let grid = Grid::<char>::parse(input);
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    let mut seen = HashSet::new();
    for (row, values) in grid.data().into_iter().enumerate() {
        for (col, &value) in values.into_iter().enumerate() {
            let coords = GridCoords { row, col };
            if seen.contains(&coords) {
                continue;
            }

            // Perform BFS over the region.
            let mut positions: HashSet<_> = HashSet::from_iter(std::iter::once(coords));
            let mut queue = VecDeque::from_iter(std::iter::once(coords));
            let mut oob = HashSet::new();
            while let Some(position) = queue.pop_back() {
                for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)].map(|(row, col)| GridCoords { row, col }) {
                    let offset_pos = position.with_offset(offset);
                    match grid.get_with_signed_coords(offset_pos) {
                        Some(cell) if *cell.data.value == value => {
                            let next_pos = cell.data.position;
                            if !positions.contains(&next_pos) {
                                positions.insert(next_pos);
                                queue.push_front(next_pos);
                            }
                        },
                        _ => {
                            oob.insert((position, offset));
                        },
                    }
                }
            }

            // Count the number of vertical edges in this region.
            //
            // We can leverage the geometric property that any of these region shapes will have
            // a total number of edges = the number of vertical edges * 2.
            let mut vertical_edges = 0;
            let start_row = positions.iter().map(|pos| pos.row).min().unwrap();
            let start_col = positions.iter().map(|pos| pos.col).min().unwrap();
            let end_row = positions.iter().map(|pos| pos.row).max().unwrap();
            let end_col = positions.iter().map(|pos| pos.col).max().unwrap();
            for col in start_col..=end_col {
                let mut on_edge_left = false;
                let mut on_edge_right = false;
                for row in start_row..=end_row {
                    let in_region = positions.contains(&GridCoords { row, col });
                    let empty_right = !positions.contains(&GridCoords { row, col: col + 1 });
                    let empty_left =
                        (col > 0).then(|| !positions.contains(&GridCoords { row, col: col - 1 })).unwrap_or(true);
                    let next_on_edge_left = in_region && empty_left;
                    let next_on_edge_right = in_region && empty_right;
                    if !next_on_edge_left && on_edge_left {
                        vertical_edges += 1;
                    }
                    if !next_on_edge_right && on_edge_right {
                        vertical_edges += 1;
                    }
                    on_edge_left = next_on_edge_left;
                    on_edge_right = next_on_edge_right;
                }
                if on_edge_left {
                    vertical_edges += 1;
                }
                if on_edge_right {
                    vertical_edges += 1;
                }
            }

            total_p1 += positions.len() * oob.len();
            total_p2 += positions.len() * (vertical_edges * 2);
            seen.extend(positions);
        }
    }
    (total_p1, total_p2)
}

pub fn part1(input: crate::Input) -> usize {
    solve(input).0
}

pub fn part2(input: crate::Input) -> usize {
    solve(input).1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
AAAA
BBCD
BBCC
EEEC
"#
        .trim();

        assert_eq!(part1(crate::Input::memory(input)), 140);
        assert_eq!(part2(crate::Input::memory(input)), 80);
    }

    #[test]
    fn example2() {
        let input = r#"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#
        .trim();

        assert_eq!(part1(crate::Input::memory(input)), 772);
        assert_eq!(part2(crate::Input::memory(input)), 436);
    }

    #[test]
    fn example3() {
        let input = r#"
 RRRRIICCFF
 RRRRIICCCF
 VVRRRCCFFF
 VVRCCCJFFF
 VVVVCJJCFE
 VVIVCCJJEE
 VVIIICJJEE
 MIIIIIJJEE
 MIIISIJEEE
 MMMISSJEEE
            "#
        .trim();

        assert_eq!(part1(crate::Input::memory(input)), 1930);
        assert_eq!(part2(crate::Input::memory(input)), 1206);
    }
}
