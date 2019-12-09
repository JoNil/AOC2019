use super::run_program;
use std::error::Error;

#[test]
fn run_program_test() {
    test().unwrap();
}

fn test() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8])?,
        &[1]
    );
    assert_eq!(
        run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[7])?,
        &[0]
    );

    assert_eq!(
        run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[8])?,
        &[0]
    );
    assert_eq!(
        run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[7])?,
        &[1]
    );

    assert_eq!(run_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8])?, &[1]);
    assert_eq!(run_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[7])?, &[0]);

    assert_eq!(run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[8])?, &[0]);
    assert_eq!(run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[7])?, &[1]);

    {
        let output = run_program(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0],
        )?;

        assert_eq!(output, &[0]);
    }

    {
        let output = run_program(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[2],
        )?;

        assert_eq!(output, &[1]);
    }

    {
        let output = run_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[0])?;
        assert_eq!(output, &[0]);
    }

    {
        let output = run_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[2])?;
        assert_eq!(output, &[1]);
    }

    {
        let program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(run_program(&program, &[5])?, &[999]);
        assert_eq!(run_program(&program, &[8])?, &[1000]);
        assert_eq!(run_program(&program, &[15])?, &[1001]);
    }

    {
        let program = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let output = run_program(&program, &[])?;

        assert_eq!(output, &program[..]);
    }

    {
        let program = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let output = run_program(&program, &[])?;

        assert_eq!(output, &[1219070632396864]);
    }

    {
        let program = [104, 1125899906842624, 99];

        let output = run_program(&program, &[])?;

        assert_eq!(output, &[1125899906842624]);
    }

    Ok(())
}
