use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{anychar, line_ending},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().len() as i32;

    let (_input, antennas) = parse(Span::new(input))
        .map_err(|e| miette!("Parsing failed: {}", e))?;

    let mut antinodes: HashSet<IVec2> = HashSet::new();
    antennas.iter().for_each(|antenna_locations| {
        antenna_locations
            .iter()
            .permutations(2)
            .unique()
            .for_each(|antenna_pair| {
                let mut harmonic = 0;
                let distance: IVec2 =
                    antenna_pair[0] - antenna_pair[1];
                loop {
                    let antinode: IVec2 = antenna_pair[0]
                        + harmonic * distance;
                    if (1..=cols).contains(&antinode.x)
                        && (1..=rows).contains(&antinode.y)
                    {
                        antinodes.insert(antinode);
                        harmonic += 1;
                    } else {
                        break;
                    }
                }
            });
    });

    Ok(antinodes.len().to_string())
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse(input: Span) -> IResult<Span, Vec<Vec<IVec2>>> {
    let (input, items) =
        separated_list1(line_ending, many1(token))(input)?;

    let antennas: Vec<(IVec2, char)> = items
        .into_iter()
        .flatten()
        .filter(|(_position, value)| {
            value.ne(&'.') && value.ne(&'\n')
        })
        .collect();

    let mut frequencies: HashMap<char, Vec<IVec2>> =
        HashMap::new();

    antennas.iter().for_each(|(position, antenna)| {
        frequencies
            .entry(*antenna)
            .or_default()
            .push(*position)
    });

    Ok((
        input,
        frequencies.values().cloned().collect(),
    ))
}

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let row = input.location_line();
    let col = input.get_column();
    let (input, token) = anychar(input)?;

    Ok((
        input,
        (
            IVec2::new(col as i32, row as i32),
            token,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
