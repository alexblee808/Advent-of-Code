#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let characters: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let mut xmas_count: i32 = 0;

    for (c, row) in characters.iter().enumerate() {
        for (r, _) in row.chars().enumerate() {
            if characters[c].chars().nth(r).unwrap() != 'X' {
                continue;
            }
            for dc in [-1, 0, 1] {
                for dr in [-1, 0, 1] {
                    if dc == 0 && dr == 0 {
                        continue;
                    }
                    if r as i32 + 3 * dr < 0
                        || c as i32 + 3 * dc < 0
                        || r as i32 + 3 * dr > row.len() as i32 - 1
                        || c as i32 + 3 * dc > characters.len() as i32 - 1
                    {
                        continue;
                    }
                    if characters[(c as i32 + 1 * dc) as usize]
                        .chars()
                        .nth((r as i32 + 1 * dr) as usize)
                        .unwrap()
                        == 'M'
                        && characters[(c as i32 + 2 * dc) as usize]
                            .chars()
                            .nth((r as i32 + 2 * dr) as usize)
                            .unwrap()
                            == 'A'
                        && characters[(c as i32 + 3 * dc) as usize]
                            .chars()
                            .nth((r as i32 + 3 * dr) as usize)
                            .unwrap()
                            == 'S'
                    {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    Ok(xmas_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
