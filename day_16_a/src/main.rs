use std::error::Error;
use std::fs;

fn fft(input: &str, phases: i32) -> Result<Vec<i32>, Box<dyn Error>> {

    let mut signal = input.trim().chars().map(|c| c.to_digit(10).map(|d| d as i32).ok_or("Not Digit")).collect::<Result<Vec<_>, _>>()?;
   
    let mut phase = signal.clone();

    let pattern = [0, 1, 0, -1];

    for _ in 0..phases {
        
        for (i, out_digit) in phase.iter_mut().enumerate() {

            let mut sum = 0;
            let mut pattern_counter = i;
            let mut pattern_digit = 0;

            for in_digit in signal.iter() {

                if pattern_counter == 0 {
                    pattern_counter = i + 1;
                    pattern_digit = (pattern_digit + 1) % pattern.len();
                }

                sum += *in_digit * pattern[pattern_digit];

                pattern_counter -= 1;
            }

            *out_digit = sum.abs() % 10;
        }

        signal = phase.clone();
    }

    Ok(signal)
}

fn main() -> Result<(), Box<dyn Error>> {

    let input = fs::read_to_string("input")?;

    let output = fft(&input, 100)?;

    println!("{:?}", &output[..8]);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::fft;

    #[test]
    fn test_fft() {
        assert_eq!(fft("12345678", 4).unwrap(), [0, 1, 0, 2, 9, 4, 9, 8]);
        assert_eq!(fft("80871224585914546619083218645595", 100).unwrap()[..8], [2, 4, 1, 7, 6, 1, 7, 6]);
        assert_eq!(fft("19617804207202209144916044189917", 100).unwrap()[..8], [7, 3, 7, 4, 5, 4, 1, 8]);
        assert_eq!(fft("69317163492948606335995924319873", 100).unwrap()[..8], [5, 2, 4, 3, 2, 1, 3, 3]);
    }
}
