use std::error::Error;
use std::fs;
use std::time::Instant;

// https://en.wikipedia.org/wiki/Exponentiation_by_squaring
fn combine(f: impl Fn(i64, i64, i64) -> i64, unit: i64, mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut r = unit;

    loop {
        if b == 0 {
            return r;
        }

        if b & 1 > 0 {
            r = f(r, a, m);
        }

        b >>= 1;
        a = f(a, a, m);
    }
}

// +  (mod m)
fn add(a: i64, b: i64, m: i64) -> i64 {
    return (m + (a + b) % m) % m;
}

// *  (mod m)
fn mul(a: i64, b: i64, m: i64) -> i64 {
    return combine(add, 0, a, b, m);
}

// ** (mod m)
fn pow(a: i64, b: i64, m: i64) -> i64 {
    return combine(mul, 1, a, b, m);
}

#[derive(Copy, Clone, Debug)]
struct Operation {
    a: i64,
    b: i64,
    len: i64,
}

fn parse_operation(input: &str, len: i64) -> Operation {
    let mut operation = Operation {
        a: 1,
        b: 0,
        len: len,
    };

    for line in input.lines() {
        let line = line.trim();

        if line.starts_with("deal with increment ") {
            if let Some(num) = line.split("deal with increment ").nth(1) {
                if let Ok(num) = num.parse::<i64>() {
                    operation.a = mul(operation.a, num, len);
                    operation.b = mul(operation.b, num, len);
                }
            }
        } else if line.starts_with("cut ") {
            if let Some(num) = line.split("cut ").nth(1) {
                if let Ok(num) = num.parse::<i64>() {
                    operation.b = add(operation.b, -num, len);
                }
            }
        } else if line.starts_with("deal into new stack") {
            operation.a = add(0, -operation.a, len);
            operation.b = add(-1, -operation.b, len);
        }
    }

    operation
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let tracked_pos = 2020;
    let stack_len = 119_315_717_514_047_i64;
    let n = 101_741_582_076_661_i64;

    let operation = parse_operation(&input, stack_len);

    let x = mul(
        operation.b,
        pow(operation.a - 1, stack_len - 2, stack_len),
        stack_len,
    );

    let res = add(
        mul(
            add(x, tracked_pos, stack_len),
            pow(pow(operation.a, stack_len - 2, stack_len), n, stack_len),
            stack_len,
        ),
        -x,
        stack_len,
    );

    println!("{:#?}", res);

    Ok(())
}
