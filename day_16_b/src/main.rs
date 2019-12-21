use std::error::Error;
use std::fs;

fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as i32).ok_or("Not Digit".into()))
        .collect::<Result<Vec<_>, _>>()
}

fn fft_digit_inner(signal: &[i32], digit: usize) -> i32 {

    let pattern = [0, 1, 0, -1];

    let mut sum = 0;
    let mut pattern_counter = digit + 1;
    let mut pattern_digit = 1;

    for in_digit in &signal[digit..] {
        if pattern_counter == 0 {
            pattern_counter = digit + 1;
            pattern_digit = (pattern_digit + 1) % pattern.len();
        }

        sum += *in_digit * pattern[pattern_digit];

        pattern_counter -= 1;
    }

    sum
}

fn fft_digit(signal: &[i32], digit: usize) -> i32 {

    let signal_len = signal.len();
    let signal_half_len = signal_len / 2;

    if signal_len % (digit * 2 + 2) == 0 && (signal_len / (digit * 2 + 2)) % 2 == 0 {

        let a = fft_digit(&signal[..signal_half_len], digit);
        let b = fft_digit(&signal[signal_half_len..], digit);

        if signal_len / (digit * 2 + 2) == 2 {
            a - b
        } else {
            a + b
        }

    } else {
        fft_digit_inner(signal, digit)
    }
}

fn fft(signal: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    res.resize_with(signal.len(), Default::default);

    let signal_len = signal.len();
    let signal_half_len = signal_len / 2;

    for (digit, out) in res[..signal_half_len].iter_mut().enumerate() {
        *out = fft_digit(&signal, digit).abs() % 10;
    }

    let mut last = 0;

    for (digit, out) in res[signal_half_len..].iter_mut().enumerate().rev() {

        last = (last + signal[signal_half_len + digit]).abs() % 10;
        *out = last;
    }

    res
}

fn fft_phases(mut signal: Vec<i32>, phases: i32) -> Vec<i32> {
    for _ in 0..phases {
        signal = fft(&signal);
    }

    signal
}

fn fft_b(input: &str, phases: i32) -> Result<Vec<i32>, Box<dyn Error>> {
    let offset = input[..8].parse::<usize>()?;

    let signal = {
        let initital_signal = parse_input(&input)?;

        let mut signal = Vec::new();
        signal.resize_with(initital_signal.len() * 10_000, Default::default);

        for i in 0..10_000 {
            signal[(i * initital_signal.len())..((i + 1) * initital_signal.len())]
                .copy_from_slice(&initital_signal);
        }

        signal
    };

    let output = fft_phases(signal, phases);

    Ok(output[offset..(offset + 8)].to_vec())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let output = fft_b(&input, 100)?;

    println!("{:?}", &output[..8]);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::fft_b;

    #[test]
    fn test_fft_b() {
        assert_eq!(
            fft_b("03036732577212944063491565474664", 100).unwrap(),
            [8, 4, 4, 6, 2, 0, 2, 6]
        );
        assert_eq!(
            fft_b("02935109699940807407585447034323", 100).unwrap(),
            [7, 8, 7, 2, 5, 2, 7, 0]
        );
        assert_eq!(
            fft_b("03081770884921959731165446850517", 100).unwrap(),
            [5, 3, 5, 5, 3, 7, 3, 1]
        );
    }
}
