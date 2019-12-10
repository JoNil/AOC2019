use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
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

fn is_visible(from: Astroid, to: Astroid, astroids: &HashSet<Astroid>) -> bool {
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

fn calculate_visible_set(astroid: Astroid, astroids: &HashSet<Astroid>) -> HashSet<Astroid> {
    let mut visible_set = HashSet::new();

    for other in astroids {
        if astroid != *other {
            if is_visible(astroid, *other, astroids) {
                visible_set.insert(*other);
            }
        }
    }

    visible_set
}

fn calculate_angle(monitoring_station: Astroid, astroid: Astroid) -> f64 {
    let x = (monitoring_station.pos[0] - astroid.pos[0]) as f64;
    let y = (monitoring_station.pos[1] - astroid.pos[1]) as f64;

    let angle = -f64::atan2(x, y);

    if angle < 0.0 {
        2.0 * std::f64::consts::PI + angle
    } else {
        angle
    }
    .abs()
}

fn sort_by_angle(monitoring_station: Astroid, astroids: &HashSet<Astroid>) -> Vec<Astroid> {
    let mut result: Vec<_> = astroids.iter().cloned().collect();

    result.sort_by(|a, b| {
        let a_angle = calculate_angle(monitoring_station, *a);
        let b_angle = calculate_angle(monitoring_station, *b);

        a_angle.partial_cmp(&b_angle).unwrap()
    });

    result
}

fn find_nth_dead_astroid(monitoring_station: Astroid, astroids: &[Astroid], n: i32) -> Astroid {
    let mut astroids_destroyed = 0;

    let mut remaning_astroids: HashSet<_> = astroids.iter().cloned().collect();

    loop {
        let visible_set = calculate_visible_set(monitoring_station, &remaning_astroids);

        if visible_set.len() + astroids_destroyed < n as usize {
            remaning_astroids = remaning_astroids
                .difference(&visible_set)
                .cloned()
                .collect();
            astroids_destroyed += visible_set.len();
            continue;
        }

        let visible_set_order = sort_by_angle(monitoring_station, &visible_set);

        return visible_set_order[n as usize - astroids_destroyed - 1];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let res = find_nth_dead_astroid(
        Astroid { pos: [20, 19] },
        &parse_astroid_positions(&input),
        200,
    );

    println!("{:?}", res);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::{calculate_angle, find_nth_dead_astroid, parse_astroid_positions, Astroid};

    #[test]
    fn test_calculate_angle() {
        {
            assert_eq!(
                calculate_angle(Astroid { pos: [11, 13] }, Astroid { pos: [10, 1] })
                    > calculate_angle(Astroid { pos: [11, 13] }, Astroid { pos: [12, 1] }),
                true
            );
        }

        {
            assert_eq!(
                calculate_angle(Astroid { pos: [11, 13] }, Astroid { pos: [11, 14] })
                    > calculate_angle(Astroid { pos: [11, 13] }, Astroid { pos: [11, 12] }),
                true
            );
        }
    }

    #[test]
    fn test_find_best_monitoring_position() {
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
                find_nth_dead_astroid(
                    Astroid { pos: [11, 13] },
                    &parse_astroid_positions(&input),
                    200
                ),
                Astroid { pos: [8, 2] }
            );
            assert_eq!(
                find_nth_dead_astroid(
                    Astroid { pos: [11, 13] },
                    &parse_astroid_positions(&input),
                    299
                ),
                Astroid { pos: [11, 1] }
            );
            assert_eq!(
                find_nth_dead_astroid(
                    Astroid { pos: [11, 13] },
                    &parse_astroid_positions(&input),
                    50
                ),
                Astroid { pos: [16, 9] }
            );
        }

        {
            let input = ".#....#####...#..
            ##...##.#####..##
            ##...#...#.#####.
            ..#.....#...###..
            ..#.#.....#....##";
            assert_eq!(
                find_nth_dead_astroid(Astroid { pos: [8, 3] }, &parse_astroid_positions(&input), 1),
                Astroid { pos: [8, 1] }
            );
            assert_eq!(
                find_nth_dead_astroid(Astroid { pos: [8, 3] }, &parse_astroid_positions(&input), 2),
                Astroid { pos: [9, 0] }
            );
            assert_eq!(
                find_nth_dead_astroid(Astroid { pos: [8, 3] }, &parse_astroid_positions(&input), 3),
                Astroid { pos: [9, 1] }
            );
        }
    }
}
