use std::convert::TryInto;
use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Default, Debug)]
struct Vec3 {
    p: [i32; 3],
}

#[derive(Copy, Clone, Default, Debug)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

fn parse_moons(input: &str) -> Result<Vec<Moon>, Box<dyn Error>> {
    let mut res = Vec::new();

    for line in input.lines() {
        let mut moon: Moon = Default::default();

        for part in line.split(",") {
            match part
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .split("=")
                .collect::<Vec<_>>()
                .as_slice()
            {
                &["x", x] => moon.pos.p[0] = x.parse()?,
                &["y", y] => moon.pos.p[1] = y.parse()?,
                &["z", z] => moon.pos.p[2] = z.parse()?,
                _ => return Err("Invalid moon".into()),
            }
        }

        res.push(moon);
    }

    Ok(res)
}

fn simulate_gravity_step_axis(moons: &mut [Moon; 4], gravitys: &mut [Vec3; 4], axis: usize) {
    gravitys.iter_mut().for_each(|v| v.p[axis] = 0);

    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let a = &moons[i];
            let b = &moons[j];

            let sign = i32::signum(a.pos.p[axis] - b.pos.p[axis]);
            gravitys[i].p[axis] -= sign;
            gravitys[j].p[axis] += sign;
        }
    }

    for (i, moon) in moons.iter_mut().enumerate() {
        moon.vel.p[axis] += gravitys[i].p[axis];
        moon.pos.p[axis] += moon.vel.p[axis];
    }
}

fn find_iterations_axis(moons: &mut [Moon; 4], gravitys: &mut [Vec3; 4]) -> [i64; 3] {
    let initial = moons.clone();

    let mut iterations: [i64; 3] = [0; 3];

    for axis in 0..3 {
        loop {
            iterations[axis] += 1;

            simulate_gravity_step_axis(moons, gravitys, axis);

            if initial[0].pos.p[axis] == moons[0].pos.p[axis]
                && initial[0].vel.p[axis] == moons[0].vel.p[axis]
                && initial[1].pos.p[axis] == moons[1].pos.p[axis]
                && initial[1].vel.p[axis] == moons[1].vel.p[axis]
                && initial[2].pos.p[axis] == moons[2].pos.p[axis]
                && initial[2].vel.p[axis] == moons[2].vel.p[axis]
                && initial[3].pos.p[axis] == moons[3].pos.p[axis]
                && initial[3].vel.p[axis] == moons[3].vel.p[axis]
            {
                break;
            }
        }
    }

    iterations
}

fn find_iterations(moons: &mut [Moon; 4], gravitys: &mut [Vec3; 4]) -> i64 {
    let loop_iterations_per_axis = find_iterations_axis(moons, gravitys);

    let a = num_integer::lcm(loop_iterations_per_axis[0], loop_iterations_per_axis[1]);

    num_integer::lcm(a, loop_iterations_per_axis[2])
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let moons = parse_moons(&input)?;

    let mut moons: [Moon; 4] = moons[..].try_into()?;

    let mut gravitys: [Vec3; 4] = {
        let mut gravitys = Vec::new();
        gravitys.resize_with(moons.len(), Default::default);
        gravitys[..].try_into()?
    };

    let iterations = find_iterations(&mut moons, &mut gravitys);

    println!("{}", iterations);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{find_iterations, parse_moons, simulate_gravity_step_axis, Moon};
    use std::convert::TryInto;

    #[test]
    fn test_find_iterations() {
        let input = "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>";

        let moons = parse_moons(input).unwrap();

        let mut moons: [Moon; 4] = moons[..].try_into().unwrap();

        let mut gravitys = {
            let mut gravitys = Vec::new();
            gravitys.resize_with(moons.len(), Default::default);
            gravitys[..].try_into().unwrap()
        };

        assert_eq!(find_iterations(&mut moons, &mut gravitys), 4686774924);
    }

    #[test]
    fn test_simulate_gravity_b() {
        let input = "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>";

        let moons = parse_moons(input).unwrap();

        let mut moons: [Moon; 4] = moons[..].try_into().unwrap();

        let mut gravitys = {
            let mut gravitys = Vec::new();
            gravitys.resize_with(moons.len(), Default::default);
            gravitys[..].try_into().unwrap()
        };

        for _ in 0..10 {
            simulate_gravity_step_axis(&mut moons, &mut gravitys, 0);
            simulate_gravity_step_axis(&mut moons, &mut gravitys, 1);
            simulate_gravity_step_axis(&mut moons, &mut gravitys, 2);
        }

        assert_eq!(moons[0].pos.p[0], -9);
        assert_eq!(moons[0].pos.p[1], -10);
        assert_eq!(moons[0].pos.p[2], 1);

        assert_eq!(moons[0].vel.p[0], -2);
        assert_eq!(moons[0].vel.p[1], -2);
        assert_eq!(moons[0].vel.p[2], -1);

        assert_eq!(moons[1].pos.p[0], 4);
        assert_eq!(moons[1].pos.p[1], 10);
        assert_eq!(moons[1].pos.p[2], 9);

        assert_eq!(moons[1].vel.p[0], -3);
        assert_eq!(moons[1].vel.p[1], 7);
        assert_eq!(moons[1].vel.p[2], -2);

        assert_eq!(moons[2].pos.p[0], 8);
        assert_eq!(moons[2].pos.p[1], -10);
        assert_eq!(moons[2].pos.p[2], -3);

        assert_eq!(moons[2].vel.p[0], 5);
        assert_eq!(moons[2].vel.p[1], -1);
        assert_eq!(moons[2].vel.p[2], -2);

        assert_eq!(moons[3].pos.p[0], 5);
        assert_eq!(moons[3].pos.p[1], -10);
        assert_eq!(moons[3].pos.p[2], 3);

        assert_eq!(moons[3].vel.p[0], 0);
        assert_eq!(moons[3].vel.p[1], -4);
        assert_eq!(moons[3].vel.p[2], 5);
    }
}
