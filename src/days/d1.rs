use crate::days::Day;
use color_eyre::eyre::Error;
use color_eyre::Result;

pub struct D1;

impl Day for D1 {
    fn run(input: Vec<u8>, part: u8) -> Result<()> {
        match part {
            1 => {
                let s = String::from_utf8(input)?;
                let result = s
                    .lines()
                    .map(line_to_number)
                    .collect::<Result<Vec<u32>>>()?
                    .into_iter()
                    .sum::<u32>();

                println!("{result}");
                Ok(())
            }
            _ => Err(Error::msg("Invalid part")),
        }
    }
}

fn line_to_number(line: &str) -> Result<u32> {
    let digits = line
        .chars()
        .filter(|f| f.is_ascii_digit())
        .map(|d| d.to_string().parse())
        .collect::<std::result::Result<Vec<u32>, std::num::ParseIntError>>()?;

    let d = match (digits.first(), digits.last()) {
        (Some(f), Some(l)) => (*f * 10) + *l,
        (Some(f), None) => *f,
        (None, None) => 0,
        _ => unreachable!(),
    };

    Ok(d)
}
