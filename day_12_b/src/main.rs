use std::error::Error;
use std::fs;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Eq, Hash, PartialEq, Default, Debug)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Default, Debug)]
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
                &["x", x] => moon.pos.x = x.parse()?,
                &["y", y] => moon.pos.y = y.parse()?,
                &["z", z] => moon.pos.z = z.parse()?,
                _ => return Err("Invalid moon".into()),
            }
        }

        res.push(moon);
    }

    Ok(res)
}

fn simulate_gravity_step(moons: &mut [Moon], iterations: i32) {
    for _ in 0..iterations {
        let mut gravitys = Vec::new();

        for (i, a) in moons.iter().enumerate() {
            let mut gravity: Vec3 = Default::default();

            for (j, b) in moons.iter().enumerate() {
                if i != j {
                    if a.pos.x < b.pos.x {
                        gravity.x += 1;
                    } else if a.pos.x > b.pos.x {
                        gravity.x -= 1;
                    }

                    if a.pos.y < b.pos.y {
                        gravity.y += 1;
                    } else if a.pos.y > b.pos.y {
                        gravity.y -= 1;
                    }

                    if a.pos.z < b.pos.z {
                        gravity.z += 1;
                    } else if a.pos.z > b.pos.z {
                        gravity.z -= 1;
                    }
                }
            }

            gravitys.push(gravity);
        }

        for (i, moon) in moons.iter_mut().enumerate() {
            moon.vel += gravitys[i];
            moon.pos += moon.vel;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut moons = parse_moons(&input)?;
    let initial = moons.clone();
    let mut iterations = 0;

    loop {
        iterations += 1;

        simulate_gravity_step(&mut moons, 1);

        if initial == moons {
            break;
        }
    }

    println!("{}", iterations);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_moons, simulate_gravity_step};

    #[test]
    fn test_parse_moon() {
        let input = "<x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>";

        let moons = parse_moons(input).unwrap();

        assert_eq!(moons.len(), 2);

        assert_eq!(moons[0].pos.x, -1);
        assert_eq!(moons[0].pos.y, 0);
        assert_eq!(moons[0].pos.z, 2);

        assert_eq!(moons[1].pos.x, 2);
        assert_eq!(moons[1].pos.y, -10);
        assert_eq!(moons[1].pos.z, -7);
    }

    #[test]
    fn test_simulate_gravity() {
        let input = "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>";

        let moons = parse_moons(input).unwrap();

        {
            let mut moons = moons.clone();

            simulate_gravity_step(&mut moons, 10);

            assert_eq!(moons[0].pos.x, -9);
            assert_eq!(moons[0].pos.y, -10);
            assert_eq!(moons[0].pos.z, 1);

            assert_eq!(moons[0].vel.x, -2);
            assert_eq!(moons[0].vel.y, -2);
            assert_eq!(moons[0].vel.z, -1);

            assert_eq!(moons[1].pos.x, 4);
            assert_eq!(moons[1].pos.y, 10);
            assert_eq!(moons[1].pos.z, 9);

            assert_eq!(moons[1].vel.x, -3);
            assert_eq!(moons[1].vel.y, 7);
            assert_eq!(moons[1].vel.z, -2);

            assert_eq!(moons[2].pos.x, 8);
            assert_eq!(moons[2].pos.y, -10);
            assert_eq!(moons[2].pos.z, -3);

            assert_eq!(moons[2].vel.x, 5);
            assert_eq!(moons[2].vel.y, -1);
            assert_eq!(moons[2].vel.z, -2);

            assert_eq!(moons[3].pos.x, 5);
            assert_eq!(moons[3].pos.y, -10);
            assert_eq!(moons[3].pos.z, 3);

            assert_eq!(moons[3].vel.x, 0);
            assert_eq!(moons[3].vel.y, -4);
            assert_eq!(moons[3].vel.z, 5);
        }
    }
}
