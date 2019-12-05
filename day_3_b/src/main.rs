use std::error::Error;
use std::fs;

fn parse_line_segment(
    pos: &mut (i32, i32),
    input: &str,
) -> Result<((i32, i32), (i32, i32)), Box<dyn Error>> {
    let (first, last) = input.split_at(1);

    let delta = last.parse::<i32>()?;

    let new_pos = match first {
        "L" => (pos.0 - delta, pos.1),
        "R" => (pos.0 + delta, pos.1),
        "U" => (pos.0, pos.1 + delta),
        "D" => (pos.0, pos.1 - delta),
        _ => return Err("Not a valid direction".into()),
    };

    let old_pos = *pos;

    *pos = new_pos;

    Ok((old_pos, new_pos))
}

fn is_horizontal(segment: &((i32, i32), (i32, i32))) -> bool {
    (segment.0).1 == (segment.1).1
}

fn is_vertical(segment: &((i32, i32), (i32, i32))) -> bool {
    (segment.0).0 == (segment.1).0
}

fn is_between(value: i32, low: i32, high: i32) -> bool {
    value > low && value < high
}

fn line_segments_intersect(
    segment_a: &((i32, i32), (i32, i32)),
    segment_b: &((i32, i32), (i32, i32)),
) -> Option<(i32, i32)> {
    if is_horizontal(segment_a) && is_vertical(segment_b) {
        let x_low = i32::min((segment_a.0).0, (segment_a.1).0);
        let x_high = i32::max((segment_a.0).0, (segment_a.1).0);

        let y_low = i32::min((segment_b.0).1, (segment_b.1).1);
        let y_high = i32::max((segment_b.0).1, (segment_b.1).1);

        if is_between((segment_a.0).1, y_low, y_high) && is_between((segment_b.0).0, x_low, x_high)
        {
            Some(((segment_b.0).0, (segment_a.0).1))
        } else {
            None
        }
    } else if is_vertical(segment_a) && is_horizontal(segment_b) {
        let x_low = i32::min((segment_b.0).0, (segment_b.1).0);
        let x_high = i32::max((segment_b.0).0, (segment_b.1).0);

        let y_low = i32::min((segment_a.0).1, (segment_a.1).1);
        let y_high = i32::max((segment_a.0).1, (segment_a.1).1);

        if is_between((segment_b.0).1, y_low, y_high) && is_between((segment_a.0).0, x_low, x_high)
        {
            Some(((segment_a.0).0, (segment_b.0).1))
        } else {
            None
        }
    } else {
        None
    }
}

fn segment_length(segment: &((i32, i32), (i32, i32))) -> i32 {
    i32::abs((segment.1).0 - (segment.0).0) + i32::abs((segment.1).1 - (segment.0).1)
}

fn wire_solver_b(input: &str) -> Result<i32, Box<dyn Error>> {
    let lines = input
        .lines()
        .map(|line| {
            let mut pos = (0, 0);
            line.trim()
                .split(",")
                .map(|v| parse_line_segment(&mut pos, v))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let line_a = lines
        .get(0)
        .ok_or("Count not find first line".to_string())?;
    let line_b = lines
        .get(1)
        .ok_or("Count not find second line".to_string())?;

    let mut min_steps = std::i32::MAX;

    for (a_index, line_segment_a) in line_a.iter().enumerate() {
        for (b_index, line_segment_b) in line_b.iter().enumerate() {
            if let Some(intersect) = line_segments_intersect(line_segment_a, line_segment_b) {
                let sum_steps_a = line_a[..a_index]
                    .iter()
                    .fold(0, |sum, segment| sum + segment_length(segment))
                    + segment_length(&(line_segment_a.0, intersect));

                let sum_steps_b = line_b[..b_index]
                    .iter()
                    .fold(0, |sum, segment| sum + segment_length(segment))
                    + segment_length(&(line_segment_b.0, intersect));

                min_steps = i32::min(min_steps, sum_steps_a + sum_steps_b);
            }
        }
    }

    Ok(min_steps)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let result = wire_solver_b(&input)?;

    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{segment_length, wire_solver_b};

    #[test]
    fn test_segment_length() {
        {
            let a = ((0, 0), (10, 0));
            assert_eq!(segment_length(&a), 10);
        }

        {
            let a = ((0, 0), (-10, 0));
            assert_eq!(segment_length(&a), 10);
        }

        {
            let a = ((0, 0), (0, 10));
            assert_eq!(segment_length(&a), 10);
        }

        {
            let a = ((0, 0), (0, -10));
            assert_eq!(segment_length(&a), 10);
        }
    }

    #[test]
    fn test_wire_solver_b() {
        {
            let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
                         U62,R66,U55,R34,D71,R55,D58,R83";

            assert_eq!(wire_solver_b(input).unwrap(), 610);
        }

        {
            let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                         U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

            assert_eq!(wire_solver_b(input).unwrap(), 410);
        }
    }
}
