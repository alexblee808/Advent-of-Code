use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut do_mul: bool = true;
    let memory_sums: i32 = input
        .lines()
        .map(|line| {
            let (sum, _do_mul) = get_memory_sum(line, do_mul);
            do_mul = _do_mul;
            sum
        })
        .sum();

    Ok(memory_sums.to_string())
}

fn get_memory_sum(memory: &str, do_mul: bool) -> (i32, bool) {
    let (products, _do_mul) = get_products(memory, do_mul);
    // debug!("Taking the sum of the products: {:?}", products);
    let sum: i32 = products.iter().sum();
    // debug!("The sum of the products is: {}", sum);
    (sum, _do_mul)
}

fn get_products(memory: &str, do_mul: bool) -> (Vec<i32>, bool) {
    // debug!("Parsing the memory string: {}", memory);
    // Create a regex that matches on the union of all commands
    // Each command and argument is captured
    // Using the "extended mode" flag to write a nicer Regex
    let re = Regex::new(
        r#"(?x)
        (mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)) |
        (?<do>do\(\)) |
        (?<dont>don't\(\))
        "#,
    )
    .unwrap();

    let mut do_mul: bool = do_mul;
    let products: Vec<i32> = re
        .captures_iter(memory)
        .map(|caps| {
            if caps.name("do").is_some() {
                do_mul = true;
                0
            } else if caps.name("dont").is_some() {
                do_mul = false;
                0
            } else if do_mul {
                // The unwraps are okay because every capture group must match if the whole
                // regex matches, and in this context, we know we have a match.
                //
                // Note that we use `caps.name("a").unwrap().as_str()` instead of
                // `&caps["a"]` because the lifetime of the former is the same as the
                // lifetime of `hay` above, but the lifetime of the latter is tied to the
                // lifetime of `caps` due to how the `Index` trait is defined.
                let mul_a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
                let mul_b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
                mul_a * mul_b
            } else {
                0
            }
        })
        .collect();

    (products, do_mul)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
