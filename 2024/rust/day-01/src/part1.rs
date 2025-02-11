#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let collection: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        left.push(collection[0]);
        right.push(collection[1]);
    }

    left.sort();
    right.sort();

    let dist: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(dist.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
