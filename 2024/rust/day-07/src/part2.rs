use miette::miette;
use nom::{
    bytes::complete::tag,
    character::{
        complete,
        complete::{newline, space1},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, equations) = parse(input)
        .map_err(|e| miette!("Parsing failed: {}", e))?;

    let result: u64 = equations
        .iter()
        .filter_map(|(value, numbers)| {
            match permute_equation(value, numbers) {
                true => Some(value),
                false => None,
            }
        })
        .sum();
    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        newline,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

fn permute_equation(
    test_value: &u64,
    numbers: &[u64],
) -> bool {
    static OPERATORS: [Operator; 3] = [
        Operator::Add,
        Operator::Multiply,
        Operator::Concatenate,
    ];

    let permutations: Vec<u64> =
        numbers.iter().skip(1).fold(
            Vec::from([numbers[0]]),
            |permutations, number| {
                let mut new_permutations: Vec<u64> =
                    Vec::with_capacity(
                        permutations.len()
                            * OPERATORS.len(),
                    );

                for operation in OPERATORS.iter() {
                    for permutation in permutations.iter() {
                        new_permutations.push(
                            operation.clone().operate(
                                permutation,
                                number,
                            ),
                        )
                    }
                }
                new_permutations
            },
        );
    permutations.contains(test_value)
}

//test_value : numbers
#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}
impl Operator {
    fn operate(self, x: &u64, y: &u64) -> u64 {
        match &self {
            Operator::Add => x + y,
            Operator::Multiply => x * y,
            Operator::Concatenate => {
                x * 10_u64.pow(length(*y, 10)) + y
            }
        }
    }
}
fn length(n: u64, base: u64) -> u32 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
