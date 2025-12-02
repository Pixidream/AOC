use std::fs::File;
use std::io::{self, BufRead, BufReader};
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

fn apply_rotation(current: i32, rot: Rotation) -> i32 {
    let delta = match rot.dir {
        Direction::Left => -rot.steps,
        Direction::Right => rot.steps,
    };

    (current + delta).rem_euclid(COUNTER_SIZE)
}

fn solve<R: BufRead>(reader: R) -> Result<u32, String> {
    let mut current_value: i32 = 50;
    let mut password: u32 = 0;

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| e.to_string())?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let rotation: Rotation = trimmed.parse()?;
        current_value = apply_rotation(current_value, rotation);

        if current_value == 0 {
            password += 1;
        }
    }

    Ok(password)
}

fn main() -> io::Result<()> {
    let file = File::open("./src/input/rotations.txt")?;
    let reader = BufReader::new(file);

    let password = solve(reader).map_err(io::Error::other)?;

    println!("Password: {password}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // not an official input. Created by me.
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
    fn test_apply_rotation_example_explained_in_statement() {
        let start = 50;

        let r1: Rotation = "L68".parse().unwrap();
        let v1 = apply_rotation(start, r1);
        assert_eq!(v1, 82);

        let r2: Rotation = "L30".parse().unwrap();
        let v2 = apply_rotation(v1, r2);
        assert_eq!(v2, 52);

        let r3: Rotation = "R48".parse().unwrap();
        let v3 = apply_rotation(v2, r3);
        assert_eq!(v3, 0);
    }

    #[test]
    fn test_test_input_password_is_2() {
        let cursor = Cursor::new(TEST_INPUT);
        let result = solve(cursor).unwrap();
        assert_eq!(result, 2);
    }
}
