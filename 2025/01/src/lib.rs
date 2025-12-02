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

fn compute_unbounded_pos(current: i32, rot: Rotation) -> i32 {
    let delta = match rot.dir {
        Direction::Left => -rot.steps,
        Direction::Right => rot.steps,
    };

    current + delta
}

fn count_zero_crossings(unbounded_pos: i32, final_pos: i32) -> i32 {
    (unbounded_pos - final_pos).div_euclid(COUNTER_SIZE)
}

fn apply_rotation(unbounded_pos: i32) -> i32 {
    unbounded_pos.rem_euclid(COUNTER_SIZE)
}

pub fn solve<R: BufRead>(reader: R) -> Result<u32, String> {
    let mut current_position: i32 = 50;
    let mut password: u32 = 0;

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| e.to_string())?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let previous_value_was_zero = current_position == 0;
        let rotation: Rotation = trimmed.parse()?;
        let unbounded_pos = compute_unbounded_pos(current_position, rotation);
        current_position = apply_rotation(unbounded_pos);

        if current_position == 0 {
            password += 1;
        } else if !previous_value_was_zero {
            password += count_zero_crossings(unbounded_pos, current_position).unsigned_abs();
        }
    }

    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    const EDGE_CASE_INPUT: &str = r#"R25
L10
R85
L200
R310
L1
R99
"#;
    const TEST_INPUT_2: &str = r#"R50
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
    fn test_apply_rotation_simple_scenario() {
        let start = 10;

        let r1: Rotation = "R15".parse().unwrap();
        let v1_unbounded = compute_unbounded_pos(start, r1);
        let v1 = apply_rotation(v1_unbounded);
        assert_eq!(v1, 25);

        let r2: Rotation = "L5".parse().unwrap();
        let v2_unbounded = compute_unbounded_pos(v1, r2);
        let v2 = apply_rotation(v2_unbounded);
        assert_eq!(v2, 20);

        let r3: Rotation = "R90".parse().unwrap();
        let v3_unbounded = compute_unbounded_pos(v2, r3);
        let v3 = apply_rotation(v3_unbounded);
        assert_eq!(v3, 10);
    }

    #[test]
    fn test_edge_case_password_is_7() {
        let cursor = Cursor::new(EDGE_CASE_INPUT);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_custom_test_input_password_is_4() {
        let cursor = Cursor::new(TEST_INPUT_2);
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
}
