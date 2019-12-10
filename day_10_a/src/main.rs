use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Astroid {
    pos: [i32; 2],
}

fn parse_astroid_positions(input: &str) -> Vec<Astroid> {
    let mut res = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                res.push(Astroid {
                    pos: [x as i32, y as i32],
                });
            }
        }
    }

    res
}

fn dist(a: [i32; 2], b: [i32; 2]) -> f64 {
    let abx = (b[0] - a[0]) as f64;
    let aby = (b[1] - a[1]) as f64;
    f64::sqrt(abx * abx + aby * aby)
}

fn is_visible(from: Astroid, to: Astroid, astroids: &[Astroid]) -> bool {
    let mut res = true;

    let ab = dist(from.pos, to.pos);

    for astroid in astroids {
        if *astroid == from || *astroid == to {
            continue;
        }

        let ac = dist(astroid.pos, to.pos);
        let bc = dist(astroid.pos, from.pos);

        if (ac + bc) - ab < 20.0 * std::f64::EPSILON {
            res = false;
            break;
        }
    }

    res
}

fn calculate_visible_count(astroid: Astroid, astroids: &[Astroid]) -> i32 {
    let mut count = 0;

    for other in astroids {
        if astroid != *other {
            if is_visible(astroid, *other, astroids) {
                count += 1;
            }
        }
    }

    count
}

fn find_best_monitoring_position(astroids: &[Astroid]) -> ([i32; 2], i32) {
    let mut max_visible_count = 0;
    let mut max_visible_astroid = Default::default();

    for astroid in astroids {
        let visible_count = calculate_visible_count(*astroid, astroids);

        if visible_count > max_visible_count {
            max_visible_count = visible_count;
            max_visible_astroid = *astroid;
        }
    }

    (max_visible_astroid.pos, max_visible_count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let res = find_best_monitoring_position(&parse_astroid_positions(&input));

    println!("{:?}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{find_best_monitoring_position, parse_astroid_positions, Astroid};

    #[test]
    fn test_parse_astroid_positions() {
        let input = ".#...
         ..#..";

        let output = [Astroid { pos: [1, 0] }, Astroid { pos: [2, 1] }];

        assert_eq!(parse_astroid_positions(&input), &output);
    }

    #[test]
    fn test_find_best_monitoring_position() {
        {
            let input = ".#..#
            .....
            #####
            ....#
            ...##";
            assert_eq!(
                find_best_monitoring_position(&parse_astroid_positions(&input)),
                ([3, 4], 8)
            );
        }

        {
            let input = "......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####";
            assert_eq!(
                find_best_monitoring_position(&parse_astroid_positions(&input)),
                ([5, 8], 33)
            );
        }

        {
            let input = "#.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.";
            assert_eq!(
                find_best_monitoring_position(&parse_astroid_positions(&input)),
                ([1, 2], 35)
            );
        }

        {
            let input = ".#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..";
            assert_eq!(
                find_best_monitoring_position(&parse_astroid_positions(&input)),
                ([6, 3], 41)
            );
        }

        {
            let input = ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##";
            assert_eq!(
                find_best_monitoring_position(&parse_astroid_positions(&input)),
                ([11, 13], 210)
            );
        }
    }
}
