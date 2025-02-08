use regex::Regex;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut produce: bool = false;

    let memory_sum: i32 = _input
        .lines()
        .map(|line| {
            if line.is_empty() {
                produce = true;
                // info!("List of rules: {:#?}", rules);
                0
            } else if !produce {
                let (x, y) = get_rule(line);
                rules
                    .entry(x)
                    .and_modify(|x_y| x_y.push(y))
                    .or_insert(vec![y; 1]);
                0
            } else if produce {
                let pages: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

                retrieve_page(&rules, pages)
            } else {
                0
            }
        })
        .sum();

    Ok(memory_sum.to_string())
}

fn get_rule(rule: &str) -> (i32, i32) {
    // debug!("Parsing: {}", rule);

    let re: Regex = Regex::new(r"(?<x>[0-9]{1,3})\|(?<y>[0-9]{1,3})").unwrap();
    let Some(capture) = re.captures(rule) else {
        println!("no match!");
        return (0, 0);
    };
    let x = capture.name("x").unwrap().as_str().parse::<i32>().unwrap();
    let y = capture.name("y").unwrap().as_str().parse::<i32>().unwrap();

    // debug!("Parsed: {}|{}", x, y);
    (x, y)
}

fn retrieve_page(rules: &HashMap<i32, Vec<i32>>, pages: Vec<i32>) -> i32 {
    // debug!("Retrieving the middle page for: {:?}", pages);
    for (i, page) in pages.iter().enumerate() {
        if let Some(rule) = rules.get(page) {
            for prev_page in &pages[0..i] {
                if rule.contains(prev_page) {
                    // debug!("Rule broken for: {}|{}", page, prev_page);
                    return 0;
                }
            }
        }
    }
    // debug!("The middle page is: {}", pages[pages.len() / 2]);
    pages[pages.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
