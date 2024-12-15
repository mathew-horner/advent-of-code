use crate::util::identity;

#[derive(Debug, Eq, PartialEq)]
struct Config {
    a: (u64, u64),
    b: (u64, u64),
    prize_location: (u64, u64),
}

impl Config {
    fn parse(chunk: &str) -> Self {
        let lines: Vec<_> = chunk.lines().collect();
        let a = lines[0].trim_start_matches("Button A:").replace("X+", "").replace("Y+", "");
        let b = lines[1].trim_start_matches("Button B:").replace("X+", "").replace("Y+", "");
        let p = lines[2].trim_start_matches("Prize:").replace("X=", "").replace("Y=", "");
        let (ax, ay) = a.trim().split_once(",").unwrap();
        let (bx, by) = b.trim().split_once(",").unwrap();
        let (px, py) = p.trim().split_once(",").unwrap();
        Self {
            a: (ax.trim().parse().unwrap(), ay.trim().parse().unwrap()),
            b: (bx.trim().parse().unwrap(), by.trim().parse().unwrap()),
            prize_location: (px.trim().parse().unwrap(), py.trim().parse().unwrap()),
        }
    }
}

fn parse(input: crate::Input) -> Vec<Config> {
    input.read_all().split("\n\n").map(|chunk| Config::parse(chunk)).collect()
}

fn min_tokens(config: Config) -> Option<u64> {
    let xa = config.a.0 as i64;
    let ya = config.a.1 as i64;
    let xb = config.b.0 as i64;
    let yb = config.b.1 as i64;
    let tx = config.prize_location.0 as i64;
    let ty = config.prize_location.1 as i64;

    // We simply need to solve for a & b in this system of linear equations:
    // a * xa + b * xb = tx
    // a * ya + b * yb = ty
    //
    // Using the method of substitution and some algebra yields the following equations:
    // b = -((ty * xa - ya * tx) / (ya * xb - yb * xa))
    // a = (tx - b * xb) / xa
    //
    // We solve for b, then use that result to solve for a.
    //
    // Given the problem description, we only count solutions where a and b are whole numbers.
    let bn = ty * xa - ya * tx;
    let bd = ya * xb - yb * xa;
    let b = -divide(bn, bd).unwrap_or(0);
    let a = divide(tx - b * xb, xa)?;
    Some((a * 3 + b) as u64)
}

fn divide(n: i64, d: i64) -> Option<i64> {
    (d != 0 && n % d == 0).then(|| n / d)
}

fn solve(input: crate::Input, f: impl Fn(Config) -> Config) -> u64 {
    parse(input).into_iter().map(f).fold(0, |total, config| total + min_tokens(config).unwrap_or(0))
}

pub fn part1(input: crate::Input) -> u64 {
    solve(input, identity)
}

pub fn part2(input: crate::Input) -> u64 {
    const N: u64 = 10000000000000;

    solve(input, |config| Config {
        prize_location: (config.prize_location.0 + N, config.prize_location.1 + N),
        ..config
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_tokens_example() {
        let config = Config::parse(
            &r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"#
            .trim(),
        );

        assert_eq!(min_tokens(config), Some(280));
    }

    #[test]
    fn multiple_solutions() {
        println!("{:?}", min_tokens(Config { a: (1, 1), b: (2, 2), prize_location: (100, 100) }));
    }
}
