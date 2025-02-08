use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let memory_sums: i32 = input.lines().map(get_memory_sum).sum();

    Ok(memory_sums.to_string())
}

fn get_memory_sum(memory: &str) -> i32 {
    let products: Vec<i32> = get_products(memory);
    // debug!("Taking the sum of the products: {:?}", products);
    let sum: i32 = products.iter().sum();
    // debug!("The sum of the products is: {}", sum);
    sum
}

fn get_products(memory: &str) -> Vec<i32> {
    // debug!("Parsing the memory string: {}", memory);
    let re: Regex = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the memory.
    let products: Vec<i32> = re
        .captures_iter(memory)
        .map(|caps| {
            // The unwraps are okay because every capture group must match if the whole
            // regex matches, and in this context, we know we have a match.
            //
            // Note that we use `caps.name("a").unwrap().as_str()` instead of
            // `&caps["a"]` because the lifetime of the former is the same as the
            // lifetime of `hay` above, but the lifetime of the latter is tied to the
            // lifetime of `caps` due to how the `Index` trait is defined.
            let mul_a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let mul_b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();

            // debug!("Parsed: mul({},{})", mul_a, mul_b);
            mul_a * mul_b
        })
        .collect();

    products
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
