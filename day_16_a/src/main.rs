use std::error::Error;
use std::fs;

fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as i32).ok_or("Not Digit".into()))
        .collect::<Result<Vec<_>, _>>()
}

fn fft_inner(signal: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    out.resize_with(signal.len(), Default::default);

    let pattern = [0, 1, 0, -1];

    for (i, out_digit) in out.iter_mut().enumerate() {
        let mut sum = 0;
        let mut pattern_counter = i + 1;
        let mut pattern_digit = 1;

        for in_digit in &signal[i..] {
            if pattern_counter == 0 {
                pattern_counter = i + 1;
                pattern_digit = (pattern_digit + 1) % pattern.len();
            }

            sum += *in_digit * pattern[pattern_digit];

            pattern_counter -= 1;
        }

        *out_digit = sum.abs() % 10;
    }

    out
}

fn fft(signal: &[i32]) -> Vec<i32> {
    if signal.len() > 4 {
        let mut out = Vec::new();
        out.resize_with(signal.len(), Default::default);

        let signal_len = signal.len();
        let signal_half_len = signal_len / 2;

        //a1 + b1
        //a2 - b2
        //a3 + b1 + b3 - b4
        //a4 + b1 + 2*b3

        //b1 + b2 + b3
        //b2 + b4
        //b3
        //b4

        //let a = fft(&signal.iter().skip(1).step_by(2).copied().collect::<Vec<_>>());
        //let b = fft(&signal.iter().step_by(2).copied().collect::<Vec<_>>());

        let a = fft(&signal[..signal_half_len]);
        let b = fft(&signal[signal_half_len..]);

        for (i, out_digit) in out[..signal_half_len].iter_mut().enumerate() {
            *out_digit = (a[i] + b[i]).abs() % 10;
        }

        for i in (signal_half_len..signal_len).rev() {
            if i + 1 < signal_len {
                out[i] = (out[i + 1] + b[i % signal_half_len]).abs() % 10;
            } else {
                out[i] = (b[i % signal_half_len]).abs() % 10;
            }
        }

        out
    } else {
        fft_inner(&signal)
    }
}

fn fft_phases(mut signal: Vec<i32>, phases: i32) -> Vec<i32> {
    for i in 0..phases {
        dbg!(i);
        signal = fft(&signal);
    }

    signal
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        fft_phases(parse_input("12345678").unwrap(), 4),
        [0, 1, 0, 2, 9, 4, 9, 8]
    );

    let input = fs::read_to_string("input")?;

    let signal = parse_input(&input)?;

    let output = fft_phases(signal, 100);

    println!("{:?}", &output[..8]);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::{fft_phases, parse_input};

    #[test]
    fn test_fft() {
        assert_eq!(
            fft_phases(parse_input("12345678").unwrap(), 4),
            [0, 1, 0, 2, 9, 4, 9, 8]
        );
        assert_eq!(
            fft_phases(
                parse_input("80871224585914546619083218645595").unwrap(),
                100
            )[..8],
            [2, 4, 1, 7, 6, 1, 7, 6]
        );
        assert_eq!(
            fft_phases(
                parse_input("19617804207202209144916044189917").unwrap(),
                100
            )[..8],
            [7, 3, 7, 4, 5, 4, 1, 8]
        );
        assert_eq!(
            fft_phases(
                parse_input("69317163492948606335995924319873").unwrap(),
                100
            )[..8],
            [5, 2, 4, 3, 2, 1, 3, 3]
        );
    }
}
