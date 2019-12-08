use std::error::Error;
use std::fs;

fn parse_images(input: &str, image_len: i32) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let res = input
        .trim()
        .as_bytes()
        .chunks(image_len as usize)
        .map(|chunk| {
            chunk
                .iter()
                .map(|byte| {
                    (*byte as char)
                        .to_digit(10)
                        .ok_or("Bad input".to_string())
                        .map(|n| n as i32)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let images = parse_images(&input, 25 * 6)?;

    let mut min_zeroes = std::i32::MAX;
    let mut min_zeroes_index = None;

    for (i, image) in images.iter().enumerate() {
        let zero_count = image.iter().filter(|d| **d == 0).count() as i32;

        if zero_count < min_zeroes {
            min_zeroes = zero_count;
            min_zeroes_index = Some(i)
        }
    }

    if let Some(index) = min_zeroes_index {
        let image = &images[index as usize];

        let one_count = image.iter().filter(|d| **d == 1).count() as i32;
        let two_count = image.iter().filter(|d| **d == 2).count() as i32;

        println!("{}", one_count * two_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_images;

    #[test]
    fn test() {
        let input = "123456789012";

        let images = parse_images(&input, 3 * 2).unwrap();

        assert_eq!(images.len(), 2);
        assert_eq!(images[0], [1, 2, 3, 4, 5, 6]);
        assert_eq!(images[1], [7, 8, 9, 0, 1, 2]);
    }
}
