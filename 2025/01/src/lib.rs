use std::io::BufRead;
use std::str::FromStr;

const COUNTER_SIZE: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation {
    dir: Direction,
    steps: i32,
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("empty line".into());
        }

        let (dir_str, rest) = s.split_at(1);

        let dir = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("invalid direction: '{dir_str}' in '{s}'")),
        };

        let steps =
            parse_steps(rest.as_bytes()).map_err(|e| format!("invalid steps in '{s}': {e}"))?;

        Ok(Rotation { dir, steps })
    }
}

fn parse_steps(bytes: &[u8]) -> Result<i32, &'static str> {
    if bytes.is_empty() {
        return Err("empty step");
    }

    let mut acc: i32 = 0;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return Err("non-digit character in steps");
        }
        acc = acc
            .checked_mul(10)
            .and_then(|v| v.checked_add((b - b'0') as i32))
            .ok_or("integer overflow while parsing steps")?;
    }
    Ok(acc)
}

fn count_zero_passages(pos: i32, rotation: Rotation) -> u32 {
    let delta = match rotation.dir {
        Direction::Left => -rotation.steps,
        Direction::Right => rotation.steps,
    };

    let start = pos;
    let end = pos + delta;

    if delta > 0 {
        let first_zero = ((start / COUNTER_SIZE) + 1) * COUNTER_SIZE;
        if first_zero > end {
            0
        } else {
            (((end - first_zero) / COUNTER_SIZE) + 1) as u32
        }
    } else if delta < 0 {
        let first_zero_at_or_below_start = (start / COUNTER_SIZE) * COUNTER_SIZE;

        let first_zero = if start % COUNTER_SIZE == 0 {
            first_zero_at_or_below_start - COUNTER_SIZE
        } else {
            first_zero_at_or_below_start
        };

        if first_zero < end {
            0
        } else {
            (((first_zero - end) / COUNTER_SIZE) + 1) as u32
        }
    } else {
        0
    }
}

pub fn solve<R: BufRead>(reader: R) -> Result<u32, String> {
    let mut pos = 50;
    let mut password = 0;

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| e.to_string())?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let rotation: Rotation = trimmed.parse()?;

        let passages = count_zero_passages(pos, rotation);
        password += passages;

        let delta = match rotation.dir {
            Direction::Left => -rotation.steps,
            Direction::Right => rotation.steps,
        };
        pos = (pos + delta).rem_euclid(COUNTER_SIZE);
    }

    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const TEST_INPUT: &str = r#"R50
L25
R75
L100
R1
L51
R49
R50
L1
"#;

    #[test]
    fn test_parse_steps() {
        assert_eq!(parse_steps(b"0").unwrap(), 0);
        assert_eq!(parse_steps(b"5").unwrap(), 5);
        assert_eq!(parse_steps(b"99").unwrap(), 99);
        assert!(parse_steps(b"").is_err());
        assert!(parse_steps(b"1a").is_err());
    }

    #[test]
    fn test_parse_rotation() {
        let r: Rotation = "L68".parse().unwrap();
        assert_eq!(r.dir, Direction::Left);
        assert_eq!(r.steps, 68);

        let r: Rotation = "R5".parse().unwrap();
        assert_eq!(r.dir, Direction::Right);
        assert_eq!(r.steps, 5);

        assert!("X10".parse::<Rotation>().is_err());
        assert!("L".parse::<Rotation>().is_err());
    }

    #[test]
    fn test_custom_input() {
        let cursor = Cursor::new(TEST_INPUT);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_r1000_from_50_hits_zero_10_times() {
        let input = "R1000\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_zero_passages_right() {
        assert_eq!(
            count_zero_passages(
                50,
                Rotation {
                    dir: Direction::Right,
                    steps: 60
                }
            ),
            1
        );
        assert_eq!(
            count_zero_passages(
                50,
                Rotation {
                    dir: Direction::Right,
                    steps: 50
                }
            ),
            1
        );
        assert_eq!(
            count_zero_passages(
                50,
                Rotation {
                    dir: Direction::Right,
                    steps: 30
                }
            ),
            0
        );
    }

    #[test]
    fn test_zero_passages_left() {
        assert_eq!(
            count_zero_passages(
                50,
                Rotation {
                    dir: Direction::Left,
                    steps: 60
                }
            ),
            1
        );
        assert_eq!(
            count_zero_passages(
                50,
                Rotation {
                    dir: Direction::Left,
                    steps: 30
                }
            ),
            0
        );
    }

    #[test]
    fn test_reddit_case_r50_l1() {
        let input = "R50\nL1\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 1, "R50 then L1 should pass 0 once");
    }

    #[test]
    fn test_reddit_case_l50_r50() {
        let input = "L50\nR50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 1, "L50 then R50 should pass 0 once");
    }

    #[test]
    fn test_reddit_case_l50_l50() {
        let input = "L50\nL50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 1, "L50 then L50 should pass 0 once");
    }

    #[test]
    fn test_reddit_case_r50_l50() {
        let input = "R50\nL50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 1, "R50 then L50 should pass 0 once");
    }

    #[test]
    fn test_reddit_case_r50_r50() {
        let input = "R50\nR50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 1, "R50 then R50 should pass 0 once");
    }

    #[test]
    fn test_reddit_case_l100_r100() {
        let input = "L100\nR100\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2, "L100 then R100 should pass 0 twice");
    }

    #[test]
    fn test_reddit_case_r100_l100() {
        let input = "R100\nL100\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2, "R100 then L100 should pass 0 twice");
    }

    #[test]
    fn test_reddit_case_l150_l50() {
        let input = "L150\nL50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2, "L150 then L50 should pass 0 twice");
    }

    #[test]
    fn test_reddit_case_l150_r50() {
        let input = "L150\nR50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2, "L150 then R50 should pass 0 twice");
    }

    #[test]
    fn test_reddit_case_r150_l50() {
        let input = "R150\nL50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2, "R150 then L50 should pass 0 twice");
    }

    #[test]
    fn test_reddit_case_r150_r50() {
        let input = "R150\nR50\n";
        let cursor = Cursor::new(input);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2, "R150 then R50 should pass 0 twice");
    }
}
