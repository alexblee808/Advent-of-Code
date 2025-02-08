#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let characters: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let mut xmas_count: i32 = 0;

    for (c, row) in characters.iter().enumerate() {
        for (r, _) in row.chars().enumerate() {
            // Add guard against boarder
            if c == 0 || r == 0 || c == characters.len() - 1 || r == row.len() - 1 {
                continue;
            }
            if characters[c].chars().nth(r).unwrap() != 'A' {
                continue;
            }

            let neighbors: String = [
                characters[c - 1].chars().nth(r - 1).unwrap(),
                characters[c - 1].chars().nth(r + 1).unwrap(),
                characters[c + 1].chars().nth(r + 1).unwrap(),
                characters[c + 1].chars().nth(r - 1).unwrap(),
            ]
            .iter()
            .collect();

            if neighbors == "MMSS"
                || neighbors == "MSSM"
                || neighbors == "SSMM"
                || neighbors == "SMMS"
            {
                xmas_count += 1;
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
