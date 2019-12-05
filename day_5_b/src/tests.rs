use super::run_program as run;
use std::error::Error;

#[test]
fn run_test() {
    test().unwrap();
}

fn test() -> Result<(), Box<dyn Error>> {
    {
        let mut program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        run(&mut program, &[])?;
        assert_eq!(&program, &result);
    }

    {
        let mut program = [1, 0, 0, 0, 99];
        let result = [2, 0, 0, 0, 99];

        run(&mut program, &[])?;
        assert_eq!(&program, &result);
    }

    {
        let mut program = [2, 3, 0, 3, 99];
        let result = [2, 3, 0, 6, 99];

        run(&mut program, &[])?;
        assert_eq!(&program, &result);
    }

    {
        let mut program = [2, 4, 4, 5, 99, 0];
        let result = [2, 4, 4, 5, 99, 9801];

        run(&mut program, &[])?;
        assert_eq!(&program, &result);
    }

    {
        let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = [30, 1, 1, 4, 2, 5, 6, 0, 99];

        run(&mut program, &[])?;
        assert_eq!(&program, &result);
    }

    {
        let mut program = [3, 0, 4, 0, 99];
        let result = [100, 0, 4, 0, 99];
        let input = [100];

        let output = run(&mut program, &input)?;

        assert_eq!(&program, &result);
        assert_eq!(&output, &[100]);
    }

    {
        let mut program = [1101, 100, -1, 4, 0];
        let result = [1101, 100, -1, 4, 99];

        run(&mut program, &[])?;

        assert_eq!(&program, &result);
    }

    assert_eq!(run(&mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8])?, &[1]);
    assert_eq!(run(&mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[7])?, &[0]);

    assert_eq!(run(&mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[8])?, &[0]);
    assert_eq!(run(&mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[7])?, &[1]);

    assert_eq!(run(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8])?, &[1]);
    assert_eq!(run(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &[7])?, &[0]);

    assert_eq!(run(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], &[8])?, &[0]);
    assert_eq!(run(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], &[7])?, &[1]);

    {
        let output = run(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0],
        )?;

        assert_eq!(output, &[0]);
    }

    {
        let output = run(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[2],
        )?;

        assert_eq!(output, &[1]);
    }

    {
        let output = run(&mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[0])?;
        assert_eq!(output, &[0]);
    }

    {
        let output = run(&mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[2])?;
        assert_eq!(output, &[1]);
    }

    {
        let program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(run(&mut program.to_owned(), &[5])?, &[999]);
        assert_eq!(run(&mut program.to_owned(), &[8])?, &[1000]);
        assert_eq!(run(&mut program.to_owned(), &[15])?, &[1001]);
    }

    Ok(())
}
