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
                        .ok_or("Bad input")
                        .map(|n| n as i32)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(res)
}

fn combine_layers(layers: Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut res = Vec::new();
    res.resize_with(layers.get(0).ok_or("Bad input")?.len(), || 2);

    for layer in layers.iter().rev() {
        for (i, pixel) in layer.iter().enumerate() {
            if *pixel != 2 {
                res[i] = *pixel;
            }
        }
    }

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let images = parse_images(&input, 25 * 6)?;

    let image = combine_layers(images)?;

    for line in image.chunks(25 as usize) {
        println!(
            "{}",
            line.iter()
                .map(|digit| {
                    if *digit == 1 {
                        "#".to_string()
                    } else {
                        " ".to_string()
                    }
                })
                .collect::<String>()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{combine_layers, parse_images};

    #[test]
    fn test_parse_images() {
        let input = "123456789012";

        let images = parse_images(&input, 3 * 2).unwrap();

        assert_eq!(images.len(), 2);
        assert_eq!(images[0], [1, 2, 3, 4, 5, 6]);
        assert_eq!(images[1], [7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_combine_layers() {
        let input = "0222112222120000";

        let images = parse_images(&input, 2 * 2).unwrap();
        let image = combine_layers(images).unwrap();

        assert_eq!(image, [0, 1, 1, 0]);
    }
}
