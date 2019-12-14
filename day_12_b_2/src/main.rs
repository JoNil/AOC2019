use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::ops::{AddAssign, Add, Mul};
use std::time::Instant;

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
            z: self.y + other.z,
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

impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
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

fn simulate_gravity_step(moons: &mut [Moon; 4], gravitys: &mut [Vec3; 4]) -> i64 {

    gravitys.iter_mut().for_each(|v| *v = Vec3 { x: 0, y: 0, z: 0 });
    
    for i in 0..moons.len() {
        for j in (i+1)..moons.len() {

            let a = &moons[i];
            let b = &moons[j];

            {
                let x_sign = i32::signum(a.pos.x - b.pos.x);
                gravitys[i].x -= x_sign;
                gravitys[j].x += x_sign;
            }

            {
                let y_sign = i32::signum(a.pos.y - b.pos.y);
                gravitys[i].y -= y_sign;
                gravitys[j].y += y_sign;
            }

            {
                let z_sign = i32::signum(a.pos.z - b.pos.z);
                gravitys[i].z -= z_sign;
                gravitys[j].z += z_sign;
            }
        }
    }

    let mut no_of_steps_to_next_gravity_change = std::i32::MAX;

    for i in 0..moons.len() {
        for j in (i+1)..moons.len() {

            let a = &moons[i];
            let b = &moons[j];

            {
                let p = (a.pos.x - b.pos.x) as f32;
                let v = (a.vel.x - a.vel.x) as f32;
                let a = (gravitys[i].x - gravitys[j].x) as f32;

                if a > 0.0 {
                    let x_steps = f32::floor((-v + f32::sqrt(v*v - 4.0*a*p)) / 2.0*a) as i32;
                    
                    if x_steps > 0 {
                        no_of_steps_to_next_gravity_change = no_of_steps_to_next_gravity_change.min(x_steps - 1)
                    }
                }
            }

            {
                let p = (a.pos.y - b.pos.y) as f32;
                let v = (a.vel.y - a.vel.y) as f32;
                let a = (gravitys[i].y - gravitys[j].y) as f32;

                if a > 0.0 {
                    let y_steps = f32::floor((-v + f32::sqrt(v*v - 4.0*a*p)) / 2.0*a) as i32;
                    
                    if y_steps > 0 {
                        no_of_steps_to_next_gravity_change = no_of_steps_to_next_gravity_change.min(y_steps - 1)
                    }
                }
            }

            {
                let p = (a.pos.z - b.pos.z) as f32;
                let v = (a.vel.z - a.vel.z) as f32;
                let a = (gravitys[i].z - gravitys[j].z) as f32;

                if a > 0.0 {
                    let z_steps = f32::floor((-v + f32::sqrt(v*v - 4.0*a*p)) / 2.0*a) as i32;
                    
                    if z_steps > 0 {
                        no_of_steps_to_next_gravity_change = no_of_steps_to_next_gravity_change.min(z_steps - 1)
                    }
                }
            }
        }
    }

    dbg!(no_of_steps_to_next_gravity_change);

    for (i, moon) in moons.iter_mut().enumerate() {
        moon.vel += gravitys[i] * no_of_steps_to_next_gravity_change;
        moon.pos += moon.vel * no_of_steps_to_next_gravity_change + gravitys[i] * (no_of_steps_to_next_gravity_change * no_of_steps_to_next_gravity_change);
    }

    no_of_steps_to_next_gravity_change as i64
}

fn find_iterations(moons: &mut [Moon; 4], gravitys: &mut [Vec3; 4]) -> i64 {
    
    let initial = moons.clone();
    
    let mut iterations: i64 = 0;
    let mut time = Instant::now();

    loop {

        iterations += simulate_gravity_step(moons, gravitys);

        if iterations % 10_000_000 == 0 {
            let diff = time.elapsed();
            time = Instant::now();
            println!("{:?}", diff.as_millis());
        }

        if initial == *moons {
            break;
        }
    }

    iterations
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
    use std::convert::TryInto;
    use super::{Moon, parse_moons, simulate_gravity_step, find_iterations};

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
            simulate_gravity_step(&mut moons, &mut gravitys);
        }

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
