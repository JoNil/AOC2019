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

fn wire_solver(input: &str) -> Result<i32, Box<dyn Error>> {
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

    let mut min_distance = std::i32::MAX;

    for line_segment_a in line_a {
        for line_segment_b in line_b {
            if let Some((intersect_x, intersect_y)) =
                line_segments_intersect(line_segment_a, line_segment_b)
            {
                min_distance =
                    i32::min(min_distance, i32::abs(intersect_x) + i32::abs(intersect_y));
            }
        }
    }

    Ok(min_distance)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let result = wire_solver(&input)?;

    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{line_segments_intersect, parse_line_segment, wire_solver};

    #[test]
    fn test_parse_line_segment() {
        {
            let mut last = (0, 0);
            let (_, (x, y)) = parse_line_segment(&mut last, "L75").unwrap();
            assert_eq!(x, -75);
            assert_eq!(y, 0);
        }

        {
            let mut last = (0, 0);
            let (_, (x, y)) = parse_line_segment(&mut last, "R75").unwrap();
            assert_eq!(x, 75);
            assert_eq!(y, 0);
        }

        {
            let mut last = (0, 0);
            let (_, (x, y)) = parse_line_segment(&mut last, "U75").unwrap();
            assert_eq!(x, 0);
            assert_eq!(y, 75);
        }

        {
            let mut last = (0, 0);
            let (_, (x, y)) = parse_line_segment(&mut last, "D75").unwrap();
            assert_eq!(x, 0);
            assert_eq!(y, -75);
        }
    }

    #[test]
    fn test_line_segment_intersect() {
        {
            let a = ((0, 0), (10, 0));
            let b = ((1, -1), (1, 1));

            assert_eq!(line_segments_intersect(&a, &b), Some((1, 0)));
        }

        {
            let a = ((1, -1), (1, 1));
            let b = ((0, 0), (10, 0));

            assert_eq!(line_segments_intersect(&a, &b), Some((1, 0)));
        }

        {
            let a = ((0, 1), (10, 1));
            let b = ((0, 0), (10, 0));

            assert_eq!(line_segments_intersect(&a, &b), None);
        }

        {
            let a = ((0, 0), (0, 0));
            let b = ((0, 0), (0, 0));

            assert_eq!(line_segments_intersect(&a, &b), None);
        }
    }

    #[test]
    fn test_wire_solver() {
        {
            let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
                         U62,R66,U55,R34,D71,R55,D58,R83";

            assert_eq!(wire_solver(input).unwrap(), 159);
        }

        {
            let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                         U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

            assert_eq!(wire_solver(input).unwrap(), 135);
        }
    }
}
