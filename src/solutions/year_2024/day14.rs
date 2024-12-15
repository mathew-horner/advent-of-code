use std::collections::HashSet;

use crate::util::math::Vec2;

const HEIGHT: u32 = 103;
const WIDTH: u32 = 101;

struct Robot {
    position: Vec2<u32>,
    velocity: Vec2<i32>,
}

impl Robot {
    fn parse(line: &str) -> Self {
        let (position, velocity) = line.split_once(' ').unwrap();
        let position = position.trim_start_matches("p=");
        let velocity = velocity.trim_start_matches("v=");
        let (px, py) = position.split_once(',').unwrap();
        let (vx, vy) = velocity.split_once(',').unwrap();
        let position = Vec2::new(px.parse().unwrap(), py.parse().unwrap());
        let velocity = Vec2::new(vx.parse().unwrap(), vy.parse().unwrap());
        Self { position, velocity }
    }

    fn tick(&mut self, n: usize) {
        let x = self.position.x as i32 + self.velocity.x * n as i32;
        let y = self.position.y as i32 + self.velocity.y * n as i32;
        let x = x.rem_euclid(WIDTH as i32);
        let y = y.rem_euclid(HEIGHT as i32);
        self.position.x = x as u32;
        self.position.y = y as u32;
    }
}

fn parse(input: crate::Input) -> Vec<Robot> {
    input.read_lines().map(|line| Robot::parse(&line)).collect()
}

pub fn part1(input: crate::Input) -> u32 {
    const TICKS: usize = 100;

    let mut robots = parse(input);
    robots.iter_mut().for_each(|robot| {
        robot.tick(TICKS);
    });

    let q1 = Vec2::new(WIDTH / 2 + 1..WIDTH, 0..HEIGHT / 2);
    let q2 = Vec2::new(0..WIDTH / 2, 0..HEIGHT / 2);
    let q3 = Vec2::new(0..WIDTH / 2, HEIGHT / 2 + 1..HEIGHT);
    let q4 = Vec2::new(WIDTH / 2 + 1..WIDTH, HEIGHT / 2 + 1..HEIGHT);
    let quadrants = [q1, q2, q3, q4];
    let mut counts = [0; 4];

    for robot in &robots {
        for (idx, quadrant) in quadrants.clone().into_iter().enumerate() {
            if quadrant.x.contains(&robot.position.x) && quadrant.y.contains(&robot.position.y) {
                counts[idx] += 1;
                break;
            }
        }
    }

    counts.into_iter().product()
}

pub fn part2(input: crate::Input) -> u32 {
    const IN_ROW: u32 = 16;

    let mut robots = parse(input);
    let mut tick = 1;
    'outer: loop {
        robots.iter_mut().for_each(|robot| {
            robot.tick(1);
        });
        let robot_positions: HashSet<_> = HashSet::from_iter(robots.iter().map(|robot| robot.position.clone()));
        for y in 0..HEIGHT {
            for x in 0..WIDTH - IN_ROW {
                if (x..=x + IN_ROW).all(|x| robot_positions.contains(&Vec2::new(x, y))) {
                    break 'outer;
                }
            }
        }
        tick += 1;
    }
    let positions: HashSet<_> = robots.into_iter().map(|robot| robot.position).collect();
    debug_grid(&positions);
    tick
}

fn debug_grid(positions: &HashSet<Vec2<u32>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&Vec2::new(x, y)) {
                print!("R");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
