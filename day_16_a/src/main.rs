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

    let half_size = signal.len() / 2;

    {
        let mut last = 0;

        for (i, out_digit) in out[half_size..].iter_mut().enumerate().rev() {
            last = last + signal[half_size + i];

            *out_digit = last;
        }
    }

    let div_3_size = (signal.len() - 2) / 3 + 1;

    {
        let mut sub_row = signal.len() - 1;

        let mut last = 0;

        for i in (div_3_size..half_size).rev() {
            last = last + signal[i];

            out[i] = last + out[half_size] - out[sub_row];
            
            sub_row -= 2;
        }
    }

    for out in out[div_3_size..].iter_mut() {
        *out = out.abs() % 10
    }

    {
        let pattern = [0, 1, 0, -1];

        for (i, out_digit) in out[..div_3_size].iter_mut().enumerate() {
            let mut sum = 0;
            let mut pattern_digit = 1;

            for in_digit in signal[i..].chunks(i + 1) {
                if pattern_digit == 1 || pattern_digit == 3 {
                    sum += in_digit.iter().sum::<i32>() * pattern[pattern_digit];
                }
                pattern_digit = (pattern_digit + 1) % 4;
            }

            *out_digit = sum.abs() % 10;
        }
    }

    out
}

fn fft(signal: &[i32]) -> Vec<i32> {
    fft_inner(&signal)
}

fn fft_phases(mut signal: Vec<i32>, phases: i32) -> Vec<i32> {
    for _ in 0..phases {
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
